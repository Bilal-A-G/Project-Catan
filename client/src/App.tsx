import React, { useEffect, useState } from 'react';

function App() {
  function getData(){
    fetch("http://localhost:5000/", {method: "GET", mode: "no-cors"})
    .then((res) => {
      return res.text
    })
  }
  useEffect(() => {
    console.log(getData());
  }, [])
  return (
    <div>App</div>
  );
}

export default App;
