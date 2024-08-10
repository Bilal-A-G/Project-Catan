import { width, height, hexSize, hexAxialToDC} from "./Constants";
import { Matrix3x3, Vector3 } from "./Math";

//-----------------------------------Hex axial - screen and vice versa conversions -------------------------------//
//Given a fraction axial coordinate, it returns the nearest whole axial coordinate. Example usage, given an axial coordinate of a
//vertex or edge, it retrives the axial coordinates of the hex they belong to
export function HexAxialRound(axial : Vector3) : Vector3{
    let rounded : Vector3 = new Vector3(Math.round(axial.x), Math.round(axial.y));
    let remainder : Vector3 = new Vector3(axial.x - rounded.x, axial.y - rounded.y);

    let finalRoundedAxial : Vector3 = new Vector3(0, 0);
    if (Math.abs(remainder.x) >= Math.abs(remainder.y)){
        finalRoundedAxial.x = rounded.x + Math.round(remainder.x + 0.5 * remainder.y);
        finalRoundedAxial.y = rounded.y;
    }
    else{
        finalRoundedAxial.x = rounded.x;
        finalRoundedAxial.y = rounded.y + Math.round(remainder.y + 0.5 * remainder.x);
    }

    return new Vector3(finalRoundedAxial.x - 1, finalRoundedAxial.y - 3);
}

//Convert screen space coordinates to q and r axial coordinates
export function HexScreenToAxial(screen : Vector3) : Vector3 | null{
    let inverseMat : Matrix3x3 | null = Matrix3x3.Inverse(hexAxialToDC);
    if(inverseMat == null){
        return null;
    }
    return Matrix3x3.MultiplyVec(inverseMat, screen);
}

//Convert q and r values to device coordinates
export function HexAxialToDC(axial : Vector3) : Vector3 {
    return Matrix3x3.MultiplyVec(hexAxialToDC, axial);
}

//Convert one of your q or r values to it's corresponding grid index
export function HexAxialToIndex(axial : number, gridSize : number) : number {
    return axial + gridSize;
}
//-----------------------------------End of Hex axial - screen and vice versa conversions -------------------------------//


//-----------------------------------Vertex axial - screen and vice versa conversions -------------------------------//
//Get the screen space coordinates of a vertex given the center and an i value (value between 0 and 6)
export function GetCorner(center : Vector3, i : number) : Vector3{
    let degAngle : number = 60 * i;
    let radAngle : number  = Math.PI / 180 * degAngle;
    return new Vector3(center.x + hexSize * Math.cos(radAngle), center.y + hexSize * Math.sin(radAngle)); 
}

//Get an axial offset (q, r) based on the index the vertex was created at, ie, 0 denotes the east vertex
export function GetVertexAxialOffsetFromI(i : number) : Vector3{
    if (i == 0 || i == 3){
        return new Vector3(0, 0);
    }
    else if (i == 1){
        return new Vector3(0, 1);
    }
    else if (i == 2){
        return new Vector3(1, -1);
    }
    else if (i == 4){
        return new Vector3(0, -1);
    }
    else if (i == 5){
        return new Vector3(-1, 1);
    }
    else{
        console.error("ERROR, index out of bounds!")
        return new Vector3(-100, -100);
    }
}

//Convert vertex screen space coordinates to q and r coordinates
export function VertexScreenToAxial(screen : Vector3, i : number) : Vector3 | null{
    let hexAxial : Vector3 | null = HexScreenToAxial(screen);
    if (hexAxial == null){
        return null;
    }
    let centerHexAxial : Vector3 | null = HexAxialRound(hexAxial);
    let axialOffset : Vector3 = GetVertexAxialOffsetFromI(i);

    return new Vector3(centerHexAxial.x + axialOffset.x, centerHexAxial.y + axialOffset.y);
}

export function VertexAxialToIndex(axial : number, gridSize : number) : number {
    return axial + gridSize + 2;
}
//-----------------------------------End of Vertex axial - screen and vice versa conversions -------------------------------//
  
//Convert the grid size to the size of an array dimension
export function ArraySizeFromGridSize(gridSize : number) : number{
    return gridSize * 2 + 1;
}

//Roll 2 dice, each dice has 6 sides
//Since random returns a number between 0 and 1(exclusive), 
//we need to multiply by 5 and add 1 to get a range between 1 and 6(exclusive)
export function RollDice() : number {
    return Math.round((Math.random() * 5 + 1) + (Math.random() * 5 + 1));
}