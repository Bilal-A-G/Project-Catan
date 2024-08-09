import { width, height, hexSize} from "./Constants";
import { Vector2 } from "./Math";

//-----------------------------------Hex axial - screen and vice versa conversions -------------------------------//
//Given a fraction axial coordinate, it returns the nearest whole axial coordinate. Example usage, given an axial coordinate of a
//vertex or edge, it retrives the axial coordinates of the hex they belong to
export function HexAxialRound(axial : Vector2) : Vector2{
    let rounded : Vector2 = new Vector2(Math.round(axial.x), Math.round(axial.y));
    let remainder : Vector2 = new Vector2(axial.x - rounded.x, axial.y - rounded.y);

    let finalRoundedAxial : Vector2 = new Vector2(0, 0);
    if (Math.abs(remainder.x) >= Math.abs(remainder.y)){
        finalRoundedAxial.x = rounded.x + Math.round(remainder.x + 0.5 * remainder.y);
        finalRoundedAxial.y = rounded.y;
    }
    else{
        finalRoundedAxial.x = rounded.x;
        finalRoundedAxial.y = rounded.y + Math.round(remainder.y + 0.5 * remainder.x);
    }

    return new Vector2(finalRoundedAxial.x - 1, finalRoundedAxial.y - 3);
}

//Convert screen space coordinates to q and r axial coordinates
export function HexScreenToAxial(screen : Vector2, spacing : Vector2, offset : Vector2) : Vector2{
    let axialX : number = (screen.x - offset.x) / spacing.x;
    let axialY : number = (screen.y - offset.y - axialX * (spacing.y/2))/spacing.y;
    return new Vector2(axialX, axialY);
}

//Convert q and r values to screen space coordinates
export function HexAxialToScreen(axial : Vector2, spacing : Vector2, offset : Vector2) : Vector2 {
    return new Vector2(offset.x + axial.x * spacing.x, offset.y + (axial.y + axial.x / 2) * spacing.y);
}

//Convert one of your q or r values to it's corresponding grid index
export function HexAxialToIndex(axial : number, gridSize : number) : number {
    return axial + gridSize;
}
//-----------------------------------End of Hex axial - screen and vice versa conversions -------------------------------//


//-----------------------------------Vertex axial - screen and vice versa conversions -------------------------------//
//Get the screen space coordinates of a vertex given the center and an i value (value between 0 and 6)
export function GetCorner(center : Vector2, i : number) : Vector2{
    let degAngle : number = 60 * i;
    let radAngle : number  = Math.PI / 180 * degAngle;
    return new Vector2(center.x + hexSize * Math.cos(radAngle), center.y + hexSize * Math.sin(radAngle)); 
}

//Get an axial offset (q, r) based on the index the vertex was created at, ie, 0 denotes the east vertex
export function GetVertexAxialOffsetFromI(i : number) : Vector2{
    if (i == 0 || i == 3){
        return new Vector2(0, 0);
    }
    else if (i == 1){
        return new Vector2(0, 1);
    }
    else if (i == 2){
        return new Vector2(1, -1);
    }
    else if (i == 4){
        return new Vector2(0, -1);
    }
    else if (i == 5){
        return new Vector2(-1, 1);
    }
    else{
        console.error("ERROR, index out of bounds!")
        return new Vector2(-100, -100);
    }
}

//Convert vertex screen space coordinates to q and r coordinates
export function VertexScreenToAxial(screen : Vector2, spacing : Vector2, offset : Vector2, i : number){
    let centerHexAxial : Vector2 = HexAxialRound(HexScreenToAxial(screen, spacing, offset));
    let axialOffset : Vector2 = GetVertexAxialOffsetFromI(i);

    return new Vector2(centerHexAxial.x + axialOffset.x, centerHexAxial.y + axialOffset.y);
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