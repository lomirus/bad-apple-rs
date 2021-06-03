const encoder = new TextEncoder();
const decoder = new TextDecoder("utf-8");

const binary = await Deno.readFile("src/source.txt");
const source = decoder.decode(binary);

const [FRAMES, ROWS, COLS] = getSize(source);

let data = source
    .split("\n\n")
    .map(matrix => matrix.split("\n").map(row => `        ${row}`).join('],\n'))
    .join('],\n    ],\n    [\n')
    .replaceAll("        ", "        [")
    .replace(/([#-]{8}\])/g, (_, s) => `${charToNumber(s).toString().padStart(3)}]`)
    .replace(/([#-]{8})/g, (_, s) => `${charToNumber(s).toString().padStart(3)}, `)
data = `pub const DATA: [[[u8; ${COLS/8}]; ${ROWS}]; ${FRAMES}] = [\n    [\n${data}]\n    ]\n];`

await Deno.writeFile("src/data.rs", encoder.encode(data));

function getSize(source: string) {
    const frames = source.split("\n\n");
    const framesLength = frames.length;
    const rows = frames[0].split('\n');
    const rowsLength = rows.length;
    const cols = rows[0];
    const colsLength = cols.length;
    return [framesLength, rowsLength, colsLength]
}

function charToNumber(str: string): number {
    const chars = str.split("");
    let n = 0;
    for (let i = 0; i < 8; i++) {
        if (chars[i] === '#') {
            n += 2 ** (7 - i)
        }
    }
    return n;
}

