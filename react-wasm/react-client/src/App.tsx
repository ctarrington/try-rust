import React, { useState } from 'react';
import './App.css';

function App() {
  import('wasm').then(({ Person, add_two_ints, greet, format_name}) => {
    setSum(add_two_ints(10, 20));
    setGreeting(greet('Rusty'));
    setName(format_name(Person.new('Fred', 'Fredrickson')));

  });

  const [sum, setSum] = useState<number>(0);
  const [greeting, setGreeting] = useState<string>('');
  const [name, setName] = useState<string>('');

  return (
    <div className="App" >
      <div>Sum Results: {sum}</div>
      <div>Greeting: {greeting} </div>
      <div>Name: {name} </div>
    </div>
  );
}

export default App;
