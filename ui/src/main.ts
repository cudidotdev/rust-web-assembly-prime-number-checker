import { is_prime } from "./pkg";

function app() {
  let query = new URLSearchParams(window.location.search);

  let num = query.get("num")?.trim();

  if (num === undefined) return;

  let wasmAnsElem = document.getElementById("wasmAns")!;
  let wasmTimeElem = document.getElementById("wasmTime")!;
  let jsAnsElem = document.getElementById("jsAns")!;
  let jsTimeElem = document.getElementById("jsTime")!;

  let wasmStartTime = Date.now();
  let wasmResult = is_prime(num);
  let wasmTime = Date.now() - wasmStartTime;

  let jsStartTime = Date.now();
  let jsResult = isPrime(Number(num));
  let jsTime = Date.now() - jsStartTime;

  wasmAnsElem.textContent = wasmResult + "";
  wasmTimeElem.textContent = wasmTime + " ms";

  jsAnsElem.textContent = jsResult + "";
  jsTimeElem.textContent = jsTime + " ms";
}

function isPrime(num: number) {
  if (!Number.isInteger(num) || num <= 1) {
    return false;
  }

  let i = 2;

  while (i * i <= num) {
    if (num % i == 0) {
      return false;
    }

    i += 1;
  }

  return true;
}

app();
