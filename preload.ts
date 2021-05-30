const encoder = new TextEncoder();
const decoder = new TextDecoder("utf-8");

const binary = await Deno.readFile("src/source.rs");
const source = decoder.decode(binary);
const data = source
    .replaceAll("        ", "        [")
    .replaceAll("#\n", "true ],\n")
    .replaceAll("-\n", "false],\n")
    .replaceAll("#", "true,  ")
    .replaceAll("-", "false, ")
await Deno.writeFile("src/data.rs", encoder.encode(data));