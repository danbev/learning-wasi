//const rust = import('./pkg');
import init, {greet} from "./pkg";

console.log("init", init);
console.log("greet", greet);

init().then((_exports) => {
  console.log("init then...");
  greet("bajja");
});
