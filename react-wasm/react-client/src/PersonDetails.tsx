import React from "react";

// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { Person } from "wasm";

type PersonProps = {
  person: Person;
};

function PersonDetails({ person }: PersonProps) {
  return (
    <div className="PersonDetails">
      <span>First Name: {person.first_name}</span>,
      <span>Last Name: {person.last_name}</span>,
      <span>Coins: {person.coins}</span>
    </div>
  );
}

export default PersonDetails;
