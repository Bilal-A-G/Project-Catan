import {describe, test, it, expect} from "vitest"
import { ArraySizeFromGridSize, CoordinateToIndex, IndexToCoordinate } from "../src/App";

describe('Array Size from Grid Size', () => {
    it('should return n = gridSize * 2 + 1, that can be used to create an array with n elements', () => {
        const gridSize = 3;
        let arraySize = ArraySizeFromGridSize(gridSize);

        expect(arraySize).toBe(7);
    })
})

describe('Coordinate to Index', () => {
    it('should return a range between 0 and gridSize * 2 + 1, that can be used to index an array dimension', () => {
        const gridSize = 3;
        for(let i = -gridSize; i < gridSize + 1; i++){
            let index = CoordinateToIndex(i, gridSize);

            expect(index).toBeGreaterThanOrEqual(0);
            expect(index).toBeLessThanOrEqual(gridSize * 2);
        }
    })
})

describe('Index to Coordinate', () => {
    it('should return a range between -gridSize and gridSize + 1, that can be used as the q or r value', () => {
        const gridSize = 3;
        for (let i = -gridSize; i < gridSize + 1; i++){
            let index = CoordinateToIndex(i, gridSize);
            let q = IndexToCoordinate(index, gridSize);

            expect(q).toBeGreaterThanOrEqual(-gridSize);
            expect(q).toBeLessThanOrEqual(gridSize);
        }
    })
})

