import {describe, test, it, expect} from "vitest"
import { Matrix2x2, Vector2 } from "../src/Math";

describe('Matrix Multiply', () => {
    it('should return [1, 0][0, 1] when multipying [4, 7][2, 6] and [0.6, -0.7][-0.2, 0.4]', () => {
        const a : Matrix2x2 = new Matrix2x2(new Vector2(4, 7), 
                                            new Vector2(2, 6));

        const b : Matrix2x2 = new Matrix2x2(new Vector2(0.6, -0.7), 
                                            new Vector2(-0.2, 0.4));

        const correctAnswer : Matrix2x2 = new Matrix2x2(new Vector2(1, 0), 
                                                        new Vector2(0, 1));

        expect(Matrix2x2.Equals(Matrix2x2.MultiplyMat(a, b), correctAnswer)).toBeTruthy();
    })
})

describe('Matrix Inverse', () => {
    it('should return the identity matrix when multiplied by inverse', () => {
        const matrix : Matrix2x2 = new Matrix2x2(new Vector2(Math.random() * 5, Math.random() * 5), 
                                                 new Vector2(Math.random() * 5, Math.random() * 5));

        const inverse : Matrix2x2 = Matrix2x2.Inverse(matrix);

        const identity : Matrix2x2 = new Matrix2x2(new Vector2(1, 0), 
                                                   new Vector2(0, 1));

        expect(Matrix2x2.Equals(Matrix2x2.MultiplyMat(matrix, inverse), identity)).toBeTruthy();
    })
})
