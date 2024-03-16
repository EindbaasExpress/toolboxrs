import { Person } from '../pkg/project_name'; // Import the WebAssembly module

// Create a new instance of the Person object
const person = Person.new("Alice", 30);

// Access fields using getter methods
console.log(person.name()); // Output: "Alice"
console.log(person.age());  // Output: 30

// Call methods on the object
console.log(person.greet()); // Output: "Hello, my name is Alice and I am 30 years old."
