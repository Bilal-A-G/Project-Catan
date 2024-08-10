import {describe, test, it, expect} from "vitest"
import { Matrix3x3, Vector3 } from "../src/Math";

describe('Matrix Multiply Matrix', () => {
    it('should return the correctAnswer matrix when a and b have been multiplied', () => {
        const a : Matrix3x3 = new Matrix3x3(
            new Vector3(2, 3, 1), 
            new Vector3(8, 2, 4),
            new Vector3(6, 1, 2));

        const b : Matrix3x3 = new Matrix3x3(
            new Vector3(5, 7, 9), 
            new Vector3(4, 4, 8),
            new Vector3(3, 4, 3));

        const correctAnswer : Matrix3x3 = new Matrix3x3(
            new Vector3(25, 30, 45), 
            new Vector3(60, 80, 100),
            new Vector3(40, 54, 68));
        
        const answer : Matrix3x3 = Matrix3x3.MultiplyMat(a, b);

        expect(answer.r0.x).toBeCloseTo(correctAnswer.r0.x);
        expect(answer.r0.y).toBeCloseTo(correctAnswer.r0.y);
        expect(answer.r0.z).toBeCloseTo(correctAnswer.r0.z);

        expect(answer.r1.x).toBeCloseTo(correctAnswer.r1.x);
        expect(answer.r1.y).toBeCloseTo(correctAnswer.r1.y);
        expect(answer.r1.z).toBeCloseTo(correctAnswer.r1.z);

        expect(answer.r2.x).toBeCloseTo(correctAnswer.r2.x);
        expect(answer.r2.y).toBeCloseTo(correctAnswer.r2.y);
        expect(answer.r2.z).toBeCloseTo(correctAnswer.r2.z);

    })
})

describe('Matrix Determinant', () => {
    it('should return -59', () => {
        const a : Matrix3x3 = new Matrix3x3(
            new Vector3(2, 2, 3), 
            new Vector3(2, 5, 4),
            new Vector3(7, 8, 1));
        
        const answer : number = Matrix3x3.Determinant(a);

        expect(answer).toBeCloseTo(-59);

    })
})

describe('Matrix Multiply Vector', () => {
    it('should return the correctAnswer vector', () => {
        const a : Matrix3x3 = new Matrix3x3(
            new Vector3(2, 2, 3), 
            new Vector3(2, 5, 4),
            new Vector3(7, 8, 1));
        
        const b : Vector3 = new Vector3(2, 0, 1);
        const answer : Vector3 = Matrix3x3.MultiplyVec(a, b);
        const correctAnswer : Vector3 = new Vector3(7, 8, 15);

        expect(answer.x).toBeCloseTo(correctAnswer.x);
        expect(answer.y).toBeCloseTo(correctAnswer.y);
        expect(answer.z).toBeCloseTo(correctAnswer.z);
    })
})

describe('Matrix Transpose', () => {
    it('should return the correctAnswer matrix', () => {
        const a : Matrix3x3 = new Matrix3x3(
            new Vector3(2, 3, 4), 
            new Vector3(6, 5, 3),
            new Vector3(3, 4, 8));
        
        const answer : Matrix3x3 = Matrix3x3.Transpose(a);
        
        const correctAnswer : Matrix3x3 = new Matrix3x3(
            new Vector3(2, 6, 3),
            new Vector3(3, 5, 4),
            new Vector3(4, 3, 8));

        expect(answer.r0.x).toBeCloseTo(correctAnswer.r0.x);
        expect(answer.r0.y).toBeCloseTo(correctAnswer.r0.y);
        expect(answer.r0.z).toBeCloseTo(correctAnswer.r0.z);
    
        expect(answer.r1.x).toBeCloseTo(correctAnswer.r1.x);
        expect(answer.r1.y).toBeCloseTo(correctAnswer.r1.y);
        expect(answer.r1.z).toBeCloseTo(correctAnswer.r1.z);
    
        expect(answer.r2.x).toBeCloseTo(correctAnswer.r2.x);
        expect(answer.r2.y).toBeCloseTo(correctAnswer.r2.y);
        expect(answer.r2.z).toBeCloseTo(correctAnswer.r2.z);
    })
})

describe('Matrix Inverse', () => {
    it('should return the identity matrix when inverse is multiplied by original', () => {
        const a : Matrix3x3 = new Matrix3x3(
            new Vector3(2, 3, 1), 
            new Vector3(4, 5, 6),
            new Vector3(7, 8, 9));

        const b : Matrix3x3 | null = Matrix3x3.Inverse(a);
        if(b == null){
            return;
        }
        const answer : Matrix3x3 = Matrix3x3.MultiplyMat(a, b);

        expect(answer.r0.x).toBeCloseTo(1);
        expect(answer.r0.y).toBeCloseTo(0);
        expect(answer.r0.z).toBeCloseTo(0);

        expect(answer.r1.x).toBeCloseTo(0);
        expect(answer.r1.y).toBeCloseTo(1);
        expect(answer.r1.z).toBeCloseTo(0);

        expect(answer.r2.x).toBeCloseTo(0);
        expect(answer.r2.y).toBeCloseTo(0);
        expect(answer.r2.z).toBeCloseTo(1);
    })
})
