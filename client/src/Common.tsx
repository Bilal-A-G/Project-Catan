import { width, height, hexSize} from "./Constants";
import { Vector3 } from "./Math";

export function GetCorner(center : Vector3, i : number) : Vector3{
    let degAngle : number = 60 * i;
    let radAngle : number  = Math.PI / 180 * degAngle;
    return new Vector3(center.x + hexSize * Math.cos(radAngle), center.y + hexSize * Math.sin(radAngle)); 
}

//Roll 2 dice, each dice has 6 sides
//Since random returns a number between 0 and 1(exclusive), 
//we need to multiply by 5 and add 1 to get a range between 1 and 6(exclusive)
export function RollDice() : number {
    return Math.round((Math.random() * 5 + 1) + (Math.random() * 5 + 1));
}