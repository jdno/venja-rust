import React from "react";
import Navbar from "react-bootstrap/Navbar";

const App: React.FC = () => {
  return (
    <Navbar bg="light">
      <Navbar.Brand href="">Venja</Navbar.Brand>
      <Navbar.Toggle aria-controls="basic-navbar-nav" />
    </Navbar>
  );
};

export default App;
