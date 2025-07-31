import { readFileSync } from "fs";
import { execSync } from "child_process";

// Generate `oxc_ast` documentation in JSON format
execSync(
  "cargo +nightly rustdoc -p oxc_ast -Z unstable-options --output-format json",
  { stdio: "inherit" },
);

// Read the generated documentation JSON file
const docData = JSON.parse(readFileSync("./target/doc/oxc_ast.json", "utf8"));

// Collect all public structs and enums
const items = [];
for (const item of Object.values(docData.index)) {
  if (item.visibility !== "public") continue;

  if (
    !item.name ||
    ["AstBuilder", "AstKind", "AstType", "NONE"].includes(item.name)
  )
    continue;

  if (item.inner?.struct)
    items.push({
      type: "struct",
      name: item.name,
      docs: item.docs ?? "",
      fields: getStructFields(item, docData),
    });

  if (item.inner?.enum)
    items.push({
      type: "enum",
      name: item.name,
      docs: item.docs ?? "",
      variants: getEnumVariants(item, docData),
    });
}

// Generate
const result = Object.fromEntries(
  items
    .sort((a, b) => a.name.localeCompare(b.name))
    .map((item) => [
      item.name,
      {
        docs: item.docs,
        body: item.type === "struct"
          ? generateStructDefinition(item)
          : generateEnumDefinition(item),
      },
    ]),
);

console.log(JSON.stringify(result, null, 2));

// ---

function getStructFields(item, docData) {
  const fieldIds =
    item.inner.struct.kind?.plain?.fields ?? item.inner.struct.fields ?? [];

  return fieldIds
    .map((fieldId) => docData.index[fieldId])
    .filter((field) => field?.visibility === "public" && field?.name)
    .map((field) => ({
      name: field.name,
      type: typeToString(field.inner.struct_field),
    }));
}

function getEnumVariants(item, docData) {
  if (!item.inner.enum.variants) return [];

  return item.inner.enum.variants
    .map((variantId) => docData.index[variantId])
    .filter((variant) => variant?.name)
    .map((variant) => {
      let variantStr = variant.name;

      const kind = variant.inner?.variant?.kind;
      if (kind) {
        if (kind.tuple?.length > 0) {
          // Tuple variant: VariantName(Type1, Type2, ...)
          const types = kind.tuple.map((fieldId) => {
            const field = docData.index[fieldId];
            return field?.inner?.struct_field
              ? typeToString(field.inner.struct_field)
              : "unknown";
          });
          variantStr += `(${types.join(", ")})`;
        } else if (kind.struct?.fields) {
          // Struct variant: VariantName { field1: Type1, field2: Type2, ... }
          const fields = kind.struct.fields
            .map((fieldId) => {
              const field = docData.index[fieldId];
              return field?.name && field?.inner?.struct_field
                ? `${field.name}: ${typeToString(field.inner.struct_field)}`
                : null;
            })
            .filter(Boolean);

          if (fields.length > 0) {
            variantStr += ` { ${fields.join(", ")} }`;
          }
        }
      }

      return variantStr;
    });
}

function typeToString(type) {
  if (!type) return "unknown";
  if (type.primitive) return type.primitive;

  if (type.resolved_path) {
    let result = type.resolved_path.path;

    if (type.resolved_path.args?.angle_bracketed) {
      const args = type.resolved_path.args.angle_bracketed.args
        .map((arg) => {
          if (arg.lifetime) return arg.lifetime;
          if (arg.type) return typeToString(arg.type);
          return "unknown";
        })
        .join(", ");

      if (args) result += `<${args}>`;
    }
    return result;
  }

  if (type.borrowed_ref) {
    const mut = type.borrowed_ref.is_mutable ? "mut " : "";
    const lifetime = type.borrowed_ref.lifetime
      ? `${type.borrowed_ref.lifetime} `
      : "";
    return `&${lifetime}${mut}${typeToString(type.borrowed_ref.type)}`;
  }

  if (type.generic) return type.generic;

  return "unknown";
}

// ---

function generateStructDefinition(structInfo) {
  const fields = structInfo.fields
    .map((field) => `  pub ${field.name}: ${field.type},`)
    .join("\n");

  return `struct ${structInfo.name} {\n${fields}\n}`;
}

function generateEnumDefinition(enumInfo) {
  const variants = enumInfo.variants
    .map((variant) => `  ${variant},`)
    .join("\n");

  return `enum ${enumInfo.name} {\n${variants}\n}`;
}
