import { type Ref } from 'vue'

function randstr(l: number): string {
  const chars =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  let res = "";
  for (let i = 0; i < l; i++) {
    res += chars[Math.floor(Math.random() * chars.length)];
  }
  return res;
}

const wait = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

export const scramble = async (t = "JazzCoding", title: Ref<string>) => {
  let l = t.length;
  for (let i = 0; i <= l; i++) {
    let rand = randstr(l);
    title.value = t.slice(0, i) + rand.slice(i, l);
    await wait(100);
  }
};
