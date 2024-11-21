import "../styles/style.scss";
import "../styles/style2.scss"

import { Application, Graphics } from "pixi.js";

(async () => {
    const app : Application = new Application();
    const canvas : HTMLElement | null = document.getElementById("canvas");
    if(!canvas){
        console.error("Error, canvas div not found!");
        return;
    }

    await app.init({
        resizeTo: canvas,
        backgroundAlpha: 0.9
    });

    const rectangle : Graphics = new Graphics()
        .rect(200, 200, 80, 80)
        .fill({
            color: 0xffff
        });
    
    app.stage.addChild(rectangle);
    canvas.appendChild(app.canvas);
})();