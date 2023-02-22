# week-year

```ts
import init, { has53Weeks, getDateStr } from "week-year";
await init();

console.log(has53Weeks(2020)); // true
console.log(has53Weeks(2021)); // false
console.log(getDateStr(2020, 1)); // 30.12.2019-03.01.2020
```
