//Vector 2 class, to store axial and screen space coordinates
export class Vector2{
    x : number;
    y : number;

    constructor(x : number, y : number){
        this.x = x;
        this.y = y;
    }
}

//Matrix 2x2 class used to store axial -> screen matrix and can be reversed for the screen -> axial conversion
export class Matrix2x2{
    r0 : Vector2;
    r1 : Vector2;

    constructor(r0 : Vector2, r1 : Vector2){
        this.r0 = r0;
        this.r1 = r1;
    }

    static Determinant(a : Matrix2x2) : number {
        return a.r0.x * a.r1.y - a.r0.y * a.r1.x
    }

    static MultiplyVec(a : Matrix2x2, b : Vector2) : Vector2 {
        return new Vector2(a.r0.x * b.x + a.r0.y * b.y, a.r1.x * b.x + a.r1.y * b.y);
    }

    static MultiplyMat(a : Matrix2x2, b : Matrix2x2) : Matrix2x2{
        return new Matrix2x2(new Vector2(a.r0.x * b.r0.x + a.r0.y * b.r1.x, a.r0.x * b.r0.y + a.r0.y * b.r1.y), 
                             new Vector2(a.r1.x * b.r0.x + a.r1.y * b.r1.x, a.r1.x * b.r0.y + a.r1.y * b.r1.y));
    }

    //Formatted like this so you can see the matrix values in their correct spots, 
    //ie, each vector represents a row in the matrix
    static Inverse(a : Matrix2x2) : Matrix2x2 {
        let determinant : number = Matrix2x2.Determinant(a);
        return new Matrix2x2(new Vector2(a.r1.y / determinant, -a.r0.y / determinant), 
                             new Vector2(-a.r1.x / determinant, a.r0.x / determinant));
    }

    static Equals(a : Matrix2x2, b : Matrix2x2) : boolean{
        return Math.round(a.r0.x) == Math.round(b.r0.x) && 
            Math.round(a.r1.x) == Math.round(b.r1.x) && 
            Math.round(a.r0.y) == Math.round(b.r0.y) && 
            Math.round(a.r1.y) == Math.round(b.r1.y);
    }
}