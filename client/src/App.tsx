import React, { useEffect, useState } from 'react';
import { Stage } from '@pixi/react';
import PixiApp from './pixi/Pixi';

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
    <div>
      <p>App</p>
      <Stage width={500} height={500}>
        <PixiApp/>
      </Stage>
    </div>
  );
}

export default App;
