import { readdirSync, writeFileSync } from "fs";
import path from "path";

const base = "./node_modules/mingcute_icon/svg";

const dir = readdirSync(base);

let s = "export type MingCuteIconName = ";
for (const region of dir) {
  const icons = readdirSync(path.join(base, region));

  const i2 = icons.map((icon) => region + "/" + icon.split(".svg")[0]);

  for (const i of i2) {
    s += `\n| "${i}"`;
  }
}
writeFileSync("./src/lib/mingcute_gen.ts", s);
