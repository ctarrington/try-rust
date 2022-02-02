import React, { useEffect, useState } from "react";
import { useInterval } from "usehooks-ts";
import "./App.css";

// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { Person } from "wasm";

type AppProps = {
  wasm: any;
};

function App({ wasm }: AppProps) {
  let {
    Person,
    add_two_ints,
    get_person,
    get_people,
    get_numbers,
    greet,
    format_name,
  } = wasm;

  useEffect(() => {
    setNumbers(get_numbers(5));
    setSum(add_two_ints(10, 20));
    setGreeting(greet("Rusty"));
    setName(format_name(Person.new("Fred", "Fredrickson")));
    const joe = get_person();
    joe.tick();
    setPersonName(joe.first_name());
    setPersonCoins(joe.coins());
    setPerson(joe);
    setPeople(get_people(2));
  }, [
    Person,
    add_two_ints,
    get_person,
    get_people,
    get_numbers,
    greet,
    format_name,
  ]);

  const [numbers, setNumbers] = useState<Uint32Array>(Uint32Array.from([]));
  const [sum, setSum] = useState<number>(0);
  const [greeting, setGreeting] = useState<string>("");
  const [name, setName] = useState<string>("");
  const [personName, setPersonName] = useState<string>("");
  const [personCoins, setPersonCoins] = useState<number>(0);
  const [person, setPerson] = useState<Person | null>(null);
  const [people, setPeople] = useState<[Person] | null>(null);

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
      {people && (
        <div>
          People:
          {people.map((person: any, index: number) => (
            <span key={index}>
              {person.first_name} {person.last_name} {person.coins},
            </span>
          ))}
        </div>
      )}
    </div>
  );
}

export default App;
