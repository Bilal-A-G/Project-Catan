import { Matrix3x3, Vector3 } from "./Math";

export const width : number = 800;
export const height : number = 500;
export const gridSize : number = 2;
export const hexSize : number = 35.35;

export const hexAxialToDC : Matrix3x3 = new Matrix3x3(
    Vector3.Multiply(new Vector3(3/2, 0, 0), hexSize), 
    Vector3.Multiply(new Vector3(Math.sqrt(3)/2, Math.sqrt(3), 0), hexSize), 
    new Vector3(0, 0, 1));

//Converts from default screen space (client.x and client.y) to a coordinate system where 0,0 is the center of the screen,
//which I'll call DC (device coordinates), you can inverse this to find SC(screen coordinates) 
export function CalulateSCToDC() : Matrix3x3{
    return new Matrix3x3(
        new Vector3(1, 0, -((window.innerWidth - width)/2) - width/2),
        new Vector3(0, 1, -((window.innerHeight - height)/2) - height/2),
        new Vector3(0, 0, 1));
}

//Converts from device coordinates to CC(canvas coordinates), this is used for rendering elements on a canvas,
//width and height are assumed to be the canvas' width and height
export const DCToCC : Matrix3x3 = new Matrix3x3(
    new Vector3(1, 0, width/2), 
    new Vector3(0, 1, height/2), 
    new Vector3(0, 0, 1));