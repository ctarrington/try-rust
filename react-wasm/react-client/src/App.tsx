import React, { useEffect, useState } from "react";
import { useInterval } from "usehooks-ts";
import "./App.css";

function App() {
  useEffect(() => {
    import("wasm").then(
      ({
        Person,
        add_two_ints,
        get_person,
        get_numbers,
        greet,
        format_name,
      }) => {
        setNumbers(get_numbers(5));
        setSum(add_two_ints(10, 20));
        setGreeting(greet("Rusty"));
        setName(format_name(Person.new("Fred", "Fredrickson")));
        const joe = get_person();
        joe.tick();
        setPersonName(joe.first_name());
        setPersonCoins(joe.coins());
        setPerson(joe);
      }
    );
  }, []);

  const [numbers, setNumbers] = useState<Uint32Array>(Uint32Array.from([]));
  const [sum, setSum] = useState<number>(0);
  const [greeting, setGreeting] = useState<string>("");
  const [name, setName] = useState<string>("");
  const [personName, setPersonName] = useState<string>("");
  const [personCoins, setPersonCoins] = useState<number>(0);
  const [person, setPerson] = useState<any>(null);

  useInterval(() => {
    if (person) {
      person.tick();
      setPersonCoins(person.coins());
    }
  }, 1000);

  return (
    <div className="App">
      <div>Sum Results: {sum}</div>
      <div>Greeting: {greeting} </div>
      <div>Name: {name} </div>
      <div>Person Name: {personName} </div>
      <div>Person Coins: {personCoins} </div>
      <div>Numbers: {numbers.join(",")}</div>
    </div>
  );
}

export default App;
