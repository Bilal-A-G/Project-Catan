export class Vector3{
    x : number;
    y : number;
    z : number;

    constructor(x : number, y : number, z : number = 1){
        this.x = x;
        this.y = y;
        this.z = z;
    }

    static Multiply(a : Vector3, b : number) : Vector3 {
        return new Vector3(a.x * b, a.y * b, a.z * b);
    }
}

//Matrix 3x3 class used to convert between coordinate spaces, 3x3 is the only way to support non linear operations
//such as translations
export class Matrix3x3{
    r0 : Vector3;
    r1 : Vector3;
    r2 : Vector3;

    constructor(r0 : Vector3, r1 : Vector3, r2 : Vector3){
        this.r0 = r0;
        this.r1 = r1;
        this.r2 = r2;
    }

    static Determinant(a : Matrix3x3) : number {
        return a.r0.x * (a.r1.y * a.r2.z - a.r1.z * a.r2.y) - 
        a.r0.y * (a.r1.x * a.r2.z - a.r1.z * a.r2.x) + 
        a.r0.z * (a.r1.x  * a.r2.y - a.r1.y * a.r2.x);
    }

    static MultiplyScalar(a : Matrix3x3, b : number) : Matrix3x3{
        return new Matrix3x3(
            new Vector3(a.r0.x * b, a.r0.y * b, a.r0.z * b),
            new Vector3(a.r1.x * b, a.r1.y * b, a.r1.z * b), 
            new Vector3(a.r2.x * b, a.r2.y * b, a.r2.z * b));
    }

    static MultiplyVec(a : Matrix3x3, b : Vector3) : Vector3 {
        return new Vector3(a.r0.x * b.x + a.r0.y * b.y + a.r0.z * b.z, 
            a.r1.x * b.x + a.r1.y * b.y + a.r1.z * b.z, 
            a.r2.x * b.x + a.r2.y * b.y + a.r2.z * b.z);
    }

    static MultiplyMat(a : Matrix3x3, b : Matrix3x3) : Matrix3x3{
        return new Matrix3x3(
            new Vector3(
            a.r0.x * b.r0.x + a.r0.y * b.r1.x + a.r0.z * b.r2.x, 
            a.r0.x * b.r0.y + a.r0.y * b.r1.y + a.r0.z * b.r2.y, 
            a.r0.x * b.r0.z + a.r0.y * b.r1.z + a.r0.z * b.r2.z),

            new Vector3(
            a.r1.x * b.r0.x + a.r1.y * b.r1.x + a.r1.z * b.r2.x, 
            a.r1.x * b.r0.y + a.r1.y * b.r1.y + a.r1.z * b.r2.y, 
            a.r1.x * b.r0.z + a.r1.y * b.r1.z + a.r1.z * b.r2.z),

            new Vector3(
            a.r2.x * b.r0.x + a.r2.y * b.r1.x + a.r2.z * b.r2.x, 
            a.r2.x * b.r0.y + a.r2.y * b.r1.y + a.r2.z * b.r2.y, 
            a.r2.x * b.r0.z + a.r2.y * b.r1.z + a.r2.z * b.r2.z));
    }

    static Transpose(a : Matrix3x3) : Matrix3x3 {
        return new Matrix3x3(
            new Vector3(a.r0.x, a.r1.x, a.r2.x), 
            new Vector3(a.r0.y, a.r1.y, a.r2.y), 
            new Vector3(a.r0.z, a.r1.z, a.r2.z));
    }

    static Inverse(a : Matrix3x3) : Matrix3x3 | null {
        let determinant : number = Matrix3x3.Determinant(a);

        if (determinant == 0){
            console.error("Error, cannot invert matrix with determinant 0!");
            return null;
        }

        let minors : Matrix3x3 = new Matrix3x3(
            new Vector3(a.r1.y * a.r2.z - a.r1.z * a.r2.y, a.r1.x * a.r2.z - a.r1.z * a.r2.x, a.r1.x * a.r2.y - a.r1.y * a.r2.x), 
            new Vector3(a.r0.y * a.r2.z - a.r0.z * a.r2.y, a.r0.x * a.r2.z - a.r0.z * a.r2.x, a.r0.x * a.r2.y - a.r2.x * a.r0.y), 
            new Vector3(a.r0.y * a.r1.z - a.r0.z * a.r1.y, a.r0.x * a.r1.z - a.r0.z * a.r1.x, a.r0.x * a.r1.y - a.r0.y * a.r1.x));

        let cofactors : Matrix3x3 = new Matrix3x3(
            new Vector3(Math.pow(-1, 2) * minors.r0.x, Math.pow(-1, 3) * minors.r0.y, Math.pow(-1, 4) * minors.r0.z), 
            new Vector3(Math.pow(-1, 3) * minors.r1.x, Math.pow(-1, 4) * minors.r1.y, Math.pow(-1, 5) * minors.r1.z), 
            new Vector3(Math.pow(-1, 4) * minors.r2.x, Math.pow(-1, 5) * minors.r2.y, Math.pow(-1, 6) * minors.r2.z));
        
        let adjugate : Matrix3x3 = Matrix3x3.Transpose(cofactors);

        return Matrix3x3.MultiplyScalar(adjugate, 1/determinant);
    }
}