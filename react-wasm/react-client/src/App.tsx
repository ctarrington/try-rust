import React, { useEffect, useRef, useState } from "react";
import "./App.css";

function useInterval(callback: () => void, delay: number) {
  const savedCallback = useRef(callback);

  // Remember the latest callback.
  useEffect(() => {
    savedCallback.current = callback;
  }, [callback]);

  // Set up the interval.
  useEffect(() => {
    let id = setInterval(() => {
      savedCallback.current();
    }, delay);
    return () => clearInterval(id);
  }, [delay]);
}

function App() {
  useEffect(() => {
    import("wasm").then(
      ({ Person, add_two_ints, get_joe, greet, format_name }) => {
        setSum(add_two_ints(10, 20));
        setGreeting(greet("Rusty"));
        setName(format_name(Person.new("Fred", "Fredrickson")));
        const joe = get_joe();
        joe.tick();
        setPersonName(joe.first_name());
        setPersonCoins(joe.coins());
        setPerson(joe);
      }
    );
  }, []);

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
    </div>
  );
}

export default App;
