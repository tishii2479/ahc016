pub const BEST_N: [[usize; 41]; 91] = [
    [
        5, 5, 7, 7, 7, 8, 10, 10, 11, 11, 11, 12, 13, 14, 14, 15, 15, 15, 16, 18, 20, 20, 21, 21,
        22, 25, 27, 27, 28, 32, 34, 36, 37, 46, 48, 56, 61, 65, 68, 80, 86,
    ],
    [
        5, 6, 7, 7, 8, 9, 10, 11, 11, 11, 12, 13, 13, 15, 16, 16, 16, 16, 17, 19, 20, 22, 24, 24,
        24, 26, 28, 29, 30, 34, 36, 40, 42, 47, 49, 57, 61, 73, 79, 86, 90,
    ],
    [
        5, 6, 7, 8, 8, 9, 10, 11, 12, 12, 13, 13, 14, 15, 16, 16, 17, 18, 19, 20, 21, 24, 26, 26,
        26, 28, 29, 31, 32, 35, 37, 42, 45, 47, 51, 58, 61, 77, 82, 90, 93,
    ],
    [
        5, 6, 8, 9, 9, 10, 11, 12, 13, 13, 13, 13, 14, 15, 16, 17, 17, 18, 19, 20, 21, 24, 26, 26,
        26, 32, 35, 35, 36, 38, 40, 47, 51, 51, 53, 66, 74, 80, 84, 90, 95,
    ],
    [
        5, 7, 8, 9, 9, 10, 11, 13, 14, 14, 14, 14, 15, 15, 17, 17, 18, 19, 20, 21, 22, 25, 27, 27,
        27, 33, 35, 39, 39, 41, 42, 49, 53, 53, 53, 71, 77, 82, 85, 91, 95,
    ],
    [
        5, 7, 8, 9, 9, 10, 12, 13, 14, 14, 14, 14, 15, 17, 18, 18, 19, 22, 22, 22, 24, 26, 28, 28,
        29, 33, 36, 39, 41, 42, 43, 50, 53, 58, 61, 72, 78, 82, 87, 92, 95,
    ],
    [
        5, 7, 8, 9, 9, 10, 13, 13, 14, 15, 15, 15, 15, 17, 19, 19, 20, 23, 23, 24, 24, 27, 29, 29,
        29, 35, 37, 40, 43, 43, 44, 50, 53, 61, 65, 73, 78, 85, 88, 93, 96,
    ],
    [
        5, 7, 8, 9, 9, 10, 13, 14, 15, 15, 15, 15, 16, 18, 19, 21, 22, 24, 25, 26, 27, 28, 29, 30,
        31, 35, 38, 41, 44, 45, 46, 50, 53, 65, 71, 75, 79, 86, 90, 95, 97,
    ],
    [
        5, 8, 8, 9, 9, 11, 13, 14, 15, 16, 16, 16, 17, 19, 20, 21, 23, 24, 26, 26, 28, 28, 29, 31,
        32, 36, 38, 42, 45, 47, 48, 51, 55, 66, 72, 77, 80, 89, 90, 96, 98,
    ],
    [
        5, 8, 8, 9, 10, 11, 13, 14, 15, 17, 18, 18, 18, 19, 20, 22, 23, 25, 26, 27, 28, 28, 29, 32,
        34, 36, 38, 46, 48, 50, 51, 53, 55, 67, 72, 78, 81, 90, 93, 97, 98,
    ],
    [
        5, 8, 9, 10, 10, 11, 13, 15, 16, 17, 19, 19, 19, 19, 20, 23, 25, 26, 26, 28, 29, 30, 30,
        33, 36, 37, 39, 47, 48, 52, 53, 55, 57, 67, 73, 78, 81, 91, 95, 97, 98,
    ],
    [
        6, 8, 9, 10, 10, 11, 13, 15, 16, 17, 19, 19, 19, 21, 22, 24, 26, 26, 27, 29, 30, 30, 31,
        35, 38, 41, 43, 47, 50, 52, 53, 57, 60, 68, 74, 78, 81, 91, 95, 97, 98,
    ],
    [
        6, 8, 9, 10, 11, 11, 13, 15, 17, 18, 19, 19, 19, 21, 22, 24, 26, 27, 28, 30, 31, 32, 33,
        37, 38, 42, 45, 48, 50, 53, 54, 59, 62, 70, 74, 79, 83, 92, 96, 97, 98,
    ],
    [
        6, 8, 10, 10, 11, 12, 14, 15, 17, 18, 19, 19, 20, 21, 22, 25, 26, 28, 28, 33, 34, 35, 35,
        37, 39, 43, 45, 48, 51, 54, 57, 61, 64, 73, 78, 83, 86, 92, 96, 97, 98,
    ],
    [
        6, 8, 10, 11, 11, 12, 14, 16, 17, 18, 19, 20, 21, 22, 23, 25, 26, 28, 29, 33, 35, 36, 36,
        38, 39, 43, 45, 48, 52, 55, 59, 63, 66, 76, 81, 86, 89, 93, 96, 97, 98,
    ],
    [
        6, 8, 10, 11, 12, 12, 14, 16, 17, 18, 19, 20, 21, 22, 23, 25, 27, 28, 30, 33, 35, 36, 36,
        38, 39, 43, 46, 50, 53, 58, 62, 68, 71, 79, 82, 90, 94, 94, 96, 98, 98,
    ],
    [
        6, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 21, 22, 22, 23, 25, 27, 29, 31, 33, 35, 36, 37,
        38, 40, 43, 46, 52, 54, 60, 63, 70, 72, 80, 83, 91, 95, 96, 96, 98, 98,
    ],
    [
        6, 9, 10, 11, 12, 13, 14, 16, 17, 18, 19, 21, 22, 23, 24, 26, 27, 29, 31, 34, 35, 36, 37,
        39, 40, 44, 47, 52, 56, 60, 63, 72, 73, 81, 83, 91, 95, 96, 97, 98, 98,
    ],
    [
        6, 9, 10, 11, 12, 13, 15, 16, 17, 18, 19, 21, 22, 23, 24, 26, 27, 29, 31, 34, 36, 37, 38,
        39, 40, 45, 48, 53, 57, 61, 64, 72, 74, 81, 83, 93, 95, 96, 97, 98, 99,
    ],
    [
        6, 9, 11, 12, 13, 14, 15, 16, 17, 19, 20, 21, 22, 23, 24, 26, 27, 29, 31, 34, 36, 37, 38,
        39, 41, 45, 48, 53, 57, 64, 66, 72, 75, 82, 84, 93, 96, 97, 97, 98, 99,
    ],
    [
        6, 9, 11, 12, 13, 14, 15, 16, 18, 19, 20, 22, 22, 24, 25, 26, 28, 29, 31, 35, 36, 37, 39,
        40, 41, 46, 49, 54, 57, 65, 68, 73, 76, 82, 85, 94, 96, 97, 98, 98, 99,
    ],
    [
        6, 9, 11, 12, 13, 15, 15, 17, 18, 20, 21, 22, 22, 24, 25, 27, 28, 30, 31, 35, 37, 37, 39,
        40, 41, 47, 50, 55, 58, 65, 69, 73, 78, 82, 85, 94, 96, 97, 98, 98, 99,
    ],
    [
        6, 9, 11, 12, 13, 15, 15, 17, 18, 20, 21, 22, 23, 24, 26, 28, 29, 31, 32, 35, 37, 37, 39,
        40, 41, 47, 50, 56, 59, 67, 70, 75, 79, 83, 89, 95, 96, 97, 98, 98, 99,
    ],
    [
        6, 9, 11, 12, 14, 15, 15, 17, 18, 20, 21, 22, 23, 25, 26, 28, 29, 31, 32, 35, 38, 38, 39,
        43, 45, 49, 52, 56, 59, 67, 71, 75, 79, 87, 93, 95, 97, 97, 98, 98, 99,
    ],
    [
        7, 10, 11, 13, 14, 15, 15, 17, 18, 20, 22, 22, 24, 26, 27, 29, 30, 32, 32, 36, 38, 39, 40,
        45, 48, 51, 53, 57, 60, 67, 71, 76, 80, 88, 93, 96, 97, 97, 98, 98, 99,
    ],
    [
        7, 10, 11, 13, 14, 15, 16, 17, 18, 20, 22, 23, 24, 26, 27, 29, 30, 32, 32, 36, 39, 40, 41,
        45, 48, 51, 53, 57, 61, 67, 72, 76, 82, 90, 96, 96, 97, 97, 98, 98, 99,
    ],
    [
        7, 10, 11, 13, 15, 15, 16, 17, 19, 20, 22, 24, 25, 26, 27, 29, 31, 32, 33, 37, 39, 42, 43,
        46, 48, 51, 53, 58, 61, 68, 72, 78, 82, 92, 96, 96, 97, 97, 98, 98, 99,
    ],
    [
        7, 10, 11, 13, 15, 15, 16, 18, 20, 20, 22, 24, 25, 27, 28, 31, 31, 32, 33, 37, 39, 43, 45,
        47, 49, 52, 53, 60, 63, 69, 73, 80, 84, 92, 96, 97, 97, 98, 98, 98, 99,
    ],
    [
        7, 10, 12, 13, 15, 16, 17, 19, 20, 21, 23, 24, 25, 27, 29, 31, 32, 32, 34, 38, 40, 43, 45,
        48, 50, 52, 54, 60, 64, 70, 74, 82, 84, 93, 97, 97, 98, 98, 98, 98, 99,
    ],
    [
        7, 10, 12, 13, 15, 16, 18, 19, 20, 22, 23, 24, 25, 28, 29, 31, 32, 34, 36, 38, 40, 43, 46,
        49, 50, 53, 54, 61, 65, 71, 74, 83, 84, 94, 97, 97, 98, 98, 98, 98, 99,
    ],
    [
        7, 10, 12, 14, 15, 16, 18, 19, 20, 22, 23, 24, 25, 28, 29, 31, 32, 35, 37, 39, 42, 44, 46,
        49, 51, 53, 55, 62, 66, 73, 76, 83, 85, 94, 97, 97, 98, 98, 98, 98, 99,
    ],
    [
        7, 10, 12, 14, 15, 16, 18, 20, 21, 22, 23, 24, 25, 28, 30, 31, 32, 36, 38, 41, 43, 45, 46,
        50, 52, 54, 55, 63, 67, 74, 79, 83, 85, 94, 97, 97, 98, 98, 98, 98, 99,
    ],
    [
        7, 10, 12, 14, 15, 16, 18, 20, 21, 23, 24, 25, 26, 29, 30, 31, 33, 36, 38, 42, 45, 45, 46,
        51, 53, 54, 56, 64, 68, 76, 80, 84, 87, 94, 97, 97, 98, 98, 98, 98, 99,
    ],
    [
        7, 10, 13, 14, 15, 17, 18, 20, 21, 24, 25, 27, 28, 29, 31, 32, 33, 36, 39, 43, 45, 46, 47,
        51, 54, 58, 61, 65, 68, 76, 80, 84, 87, 95, 97, 97, 98, 98, 98, 99, 99,
    ],
    [
        7, 10, 13, 14, 15, 17, 18, 20, 21, 24, 26, 28, 29, 29, 32, 32, 34, 37, 39, 43, 46, 47, 48,
        51, 55, 61, 64, 66, 68, 76, 80, 85, 89, 95, 97, 97, 98, 98, 99, 99, 99,
    ],
    [
        7, 10, 13, 14, 16, 17, 18, 20, 21, 24, 26, 28, 29, 31, 32, 33, 34, 37, 39, 43, 46, 47, 48,
        52, 55, 61, 64, 66, 68, 77, 81, 86, 91, 95, 97, 97, 98, 98, 99, 99, 99,
    ],
    [
        7, 10, 13, 14, 16, 17, 18, 20, 22, 24, 26, 28, 29, 31, 33, 34, 35, 38, 39, 44, 46, 48, 49,
        53, 55, 61, 64, 67, 69, 78, 83, 86, 93, 96, 97, 98, 98, 98, 99, 99, 100,
    ],
    [
        7, 11, 13, 14, 16, 18, 19, 21, 22, 25, 27, 28, 30, 32, 33, 34, 35, 38, 39, 44, 47, 48, 50,
        54, 57, 61, 64, 68, 70, 78, 83, 89, 94, 96, 97, 98, 98, 98, 99, 99, 100,
    ],
    [
        7, 11, 14, 15, 16, 18, 19, 21, 22, 25, 27, 29, 30, 32, 33, 35, 36, 38, 40, 44, 47, 49, 50,
        55, 58, 62, 64, 68, 70, 79, 85, 92, 94, 96, 97, 98, 98, 98, 99, 99, 100,
    ],
    [
        7, 11, 14, 15, 16, 19, 19, 21, 22, 25, 28, 29, 30, 32, 33, 37, 39, 39, 40, 45, 48, 49, 50,
        55, 58, 62, 64, 68, 71, 82, 86, 92, 94, 97, 97, 98, 98, 98, 99, 99, 100,
    ],
    [
        8, 11, 14, 15, 17, 19, 19, 21, 22, 26, 28, 29, 30, 33, 34, 38, 39, 40, 40, 45, 48, 50, 51,
        56, 59, 63, 65, 69, 72, 84, 88, 92, 96, 97, 98, 98, 98, 98, 99, 99, 100,
    ],
    [
        8, 11, 14, 15, 17, 19, 19, 21, 23, 26, 28, 30, 31, 33, 35, 39, 39, 40, 40, 45, 48, 50, 51,
        57, 59, 64, 65, 73, 76, 85, 90, 94, 96, 97, 98, 98, 98, 98, 99, 99, 100,
    ],
    [
        8, 11, 14, 15, 17, 19, 20, 21, 23, 26, 28, 30, 31, 34, 35, 39, 40, 40, 41, 46, 48, 51, 53,
        57, 60, 65, 67, 74, 77, 85, 90, 95, 96, 97, 98, 98, 98, 98, 99, 99, 100,
    ],
    [
        8, 11, 14, 15, 17, 19, 20, 22, 23, 26, 28, 30, 31, 34, 35, 39, 41, 41, 42, 46, 48, 52, 53,
        58, 61, 65, 67, 75, 79, 86, 90, 95, 96, 97, 98, 98, 99, 99, 99, 99, 100,
    ],
    [
        8, 11, 14, 16, 18, 19, 20, 22, 23, 27, 28, 30, 31, 34, 36, 39, 41, 41, 42, 46, 49, 52, 55,
        60, 63, 66, 69, 76, 79, 87, 90, 95, 97, 97, 98, 98, 99, 99, 99, 99, 100,
    ],
    [
        8, 11, 14, 16, 18, 19, 21, 22, 23, 27, 29, 30, 32, 34, 36, 39, 41, 41, 42, 46, 49, 53, 55,
        60, 63, 67, 69, 77, 80, 88, 92, 96, 97, 97, 98, 98, 99, 99, 99, 99, 100,
    ],
    [
        8, 11, 14, 16, 18, 20, 21, 22, 23, 27, 29, 32, 33, 34, 37, 40, 41, 42, 43, 47, 49, 53, 55,
        61, 64, 68, 72, 77, 82, 89, 92, 96, 97, 97, 98, 98, 99, 99, 99, 99, 100,
    ],
    [
        8, 11, 14, 16, 18, 20, 21, 22, 23, 27, 29, 32, 34, 37, 38, 40, 41, 42, 43, 47, 49, 54, 56,
        62, 64, 70, 72, 77, 82, 89, 93, 96, 97, 98, 98, 98, 99, 99, 100, 100, 100,
    ],
    [
        8, 11, 15, 16, 18, 21, 22, 23, 24, 27, 29, 32, 34, 37, 38, 40, 41, 43, 44, 47, 49, 54, 57,
        62, 65, 71, 74, 79, 83, 90, 93, 96, 98, 98, 98, 99, 99, 99, 100, 100, 100,
    ],
    [
        8, 11, 15, 16, 18, 21, 22, 24, 25, 27, 29, 32, 34, 37, 38, 41, 42, 44, 45, 48, 50, 54, 58,
        63, 65, 71, 74, 80, 84, 90, 93, 96, 98, 98, 98, 99, 99, 99, 100, 100, 100,
    ],
    [
        8, 11, 15, 16, 18, 21, 22, 25, 27, 27, 30, 32, 34, 37, 39, 41, 42, 46, 47, 48, 50, 55, 58,
        63, 66, 72, 75, 81, 84, 91, 94, 97, 98, 98, 98, 99, 99, 99, 100, 100, 100,
    ],
    [
        8, 12, 15, 17, 19, 21, 22, 25, 27, 29, 31, 33, 35, 37, 39, 42, 43, 46, 48, 48, 51, 56, 60,
        64, 66, 72, 76, 82, 85, 91, 94, 97, 98, 98, 98, 99, 99, 99, 100, 100, 100,
    ],
    [
        8, 12, 15, 17, 19, 21, 22, 26, 28, 30, 31, 33, 35, 38, 41, 42, 44, 46, 49, 50, 52, 57, 61,
        64, 67, 72, 77, 84, 87, 91, 95, 97, 98, 98, 99, 99, 99, 99, 100, 100, 100,
    ],
    [
        8, 12, 15, 17, 19, 21, 22, 26, 28, 30, 31, 33, 35, 38, 41, 42, 44, 46, 49, 53, 55, 59, 62,
        65, 67, 72, 77, 85, 88, 92, 95, 97, 98, 98, 99, 99, 99, 99, 100, 100, 100,
    ],
    [
        8, 12, 15, 17, 19, 21, 22, 26, 28, 31, 32, 34, 35, 39, 41, 42, 45, 47, 49, 53, 55, 59, 62,
        66, 68, 73, 77, 85, 88, 92, 95, 97, 98, 98, 99, 99, 99, 99, 100, 100, 100,
    ],
    [
        9, 12, 15, 17, 20, 21, 23, 26, 28, 31, 32, 34, 35, 39, 41, 43, 45, 47, 49, 54, 56, 60, 62,
        66, 68, 78, 82, 87, 90, 93, 95, 97, 98, 98, 99, 99, 100, 100, 100, 100, 100,
    ],
    [
        9, 12, 15, 17, 20, 22, 23, 26, 28, 31, 33, 34, 35, 39, 41, 43, 45, 48, 50, 54, 56, 60, 62,
        67, 70, 79, 83, 88, 90, 94, 96, 97, 98, 98, 99, 99, 100, 100, 100, 100, 100,
    ],
    [
        9, 12, 15, 18, 21, 22, 23, 26, 28, 31, 33, 34, 35, 40, 41, 44, 45, 48, 50, 54, 56, 60, 62,
        68, 70, 79, 83, 88, 91, 94, 96, 98, 98, 99, 99, 99, 100, 100, 100, 100, 100,
    ],
    [
        9, 12, 15, 18, 21, 22, 24, 26, 28, 31, 33, 34, 36, 40, 42, 44, 45, 49, 50, 54, 57, 61, 63,
        69, 71, 79, 84, 88, 92, 95, 97, 98, 98, 99, 99, 99, 100, 100, 100, 100, 100,
    ],
    [
        9, 13, 15, 18, 21, 22, 24, 26, 28, 31, 33, 34, 36, 40, 42, 44, 46, 50, 52, 55, 57, 61, 64,
        69, 71, 80, 85, 89, 92, 95, 97, 98, 99, 99, 99, 99, 100, 100, 100, 100, 100,
    ],
    [
        9, 13, 15, 18, 21, 22, 24, 26, 28, 31, 33, 35, 36, 40, 42, 45, 47, 50, 52, 56, 58, 61, 64,
        69, 72, 80, 85, 89, 92, 95, 98, 98, 99, 99, 99, 99, 100, 100, 100, 100, 100,
    ],
    [
        9, 13, 15, 18, 21, 22, 24, 27, 28, 31, 33, 35, 37, 40, 42, 45, 47, 51, 52, 56, 59, 62, 64,
        70, 73, 80, 85, 90, 93, 96, 98, 98, 99, 99, 99, 99, 100, 100, 100, 100, 100,
    ],
    [
        9, 13, 15, 18, 21, 23, 25, 27, 29, 31, 33, 36, 38, 40, 42, 45, 47, 51, 53, 56, 59, 63, 65,
        70, 73, 81, 85, 91, 94, 96, 98, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        9, 13, 16, 19, 21, 23, 25, 27, 29, 32, 33, 36, 38, 40, 43, 46, 48, 51, 53, 56, 59, 64, 66,
        70, 74, 81, 85, 91, 94, 96, 98, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        9, 13, 16, 19, 21, 23, 25, 27, 30, 32, 33, 36, 38, 40, 43, 46, 48, 51, 54, 56, 60, 64, 67,
        71, 74, 81, 86, 93, 94, 97, 98, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        9, 13, 16, 19, 21, 23, 26, 28, 30, 32, 34, 36, 38, 41, 43, 47, 49, 51, 54, 58, 60, 64, 67,
        72, 76, 82, 86, 93, 94, 97, 98, 99, 99, 99, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        9, 13, 16, 19, 21, 23, 26, 28, 30, 32, 34, 36, 38, 41, 44, 47, 50, 52, 54, 58, 61, 65, 67,
        73, 76, 83, 87, 93, 95, 97, 98, 99, 99, 99, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        9, 14, 16, 19, 21, 23, 26, 28, 30, 32, 34, 37, 39, 41, 44, 47, 50, 52, 55, 59, 61, 65, 67,
        74, 77, 86, 89, 93, 96, 97, 98, 99, 99, 99, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        9, 14, 16, 19, 21, 24, 26, 29, 31, 32, 34, 37, 39, 42, 45, 48, 51, 53, 55, 59, 62, 65, 68,
        74, 77, 87, 91, 93, 96, 97, 98, 99, 99, 99, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        9, 14, 16, 19, 21, 24, 26, 29, 31, 33, 35, 38, 39, 43, 45, 49, 51, 54, 56, 59, 62, 66, 69,
        74, 77, 87, 91, 95, 96, 97, 98, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 14, 16, 19, 22, 24, 27, 30, 31, 34, 36, 38, 39, 44, 47, 50, 52, 55, 58, 60, 62, 67, 70,
        74, 78, 88, 91, 95, 96, 97, 98, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 14, 17, 20, 22, 24, 27, 30, 31, 35, 36, 38, 39, 45, 47, 50, 52, 56, 58, 62, 63, 68, 70,
        75, 79, 88, 91, 95, 97, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 14, 17, 20, 22, 24, 27, 30, 32, 35, 36, 38, 40, 45, 48, 50, 52, 56, 59, 62, 64, 68, 71,
        76, 79, 88, 92, 95, 97, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 14, 17, 20, 22, 24, 27, 31, 33, 35, 36, 40, 41, 45, 48, 51, 52, 56, 59, 62, 65, 69, 71,
        77, 81, 88, 92, 95, 97, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 15, 17, 20, 22, 24, 27, 31, 33, 35, 37, 40, 41, 46, 48, 51, 53, 57, 60, 64, 67, 70, 72,
        78, 81, 88, 92, 95, 98, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 15, 17, 20, 22, 25, 27, 31, 33, 36, 39, 41, 42, 47, 48, 52, 53, 57, 60, 64, 67, 70, 72,
        79, 83, 89, 93, 96, 98, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 15, 17, 20, 22, 25, 28, 31, 33, 36, 39, 41, 43, 47, 48, 52, 53, 57, 60, 65, 69, 72, 74,
        80, 83, 89, 94, 96, 98, 98, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 15, 17, 20, 23, 25, 28, 31, 33, 37, 39, 41, 43, 47, 49, 52, 54, 58, 60, 66, 69, 72, 74,
        80, 84, 89, 94, 96, 98, 98, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 15, 17, 20, 24, 25, 28, 31, 33, 37, 39, 41, 43, 47, 49, 52, 54, 58, 60, 66, 69, 72, 74,
        81, 85, 89, 94, 96, 98, 98, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 15, 18, 20, 24, 25, 28, 31, 33, 37, 39, 42, 44, 48, 49, 53, 55, 59, 61, 66, 69, 73, 76,
        82, 85, 89, 95, 96, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 15, 18, 20, 24, 25, 28, 32, 34, 37, 39, 42, 44, 48, 49, 53, 55, 60, 61, 67, 69, 75, 78,
        82, 85, 91, 95, 97, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 15, 18, 21, 24, 25, 29, 32, 34, 37, 39, 42, 45, 48, 50, 53, 55, 60, 62, 67, 69, 75, 78,
        82, 85, 91, 95, 97, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 15, 19, 21, 24, 25, 29, 32, 34, 37, 39, 43, 45, 48, 51, 53, 55, 60, 63, 68, 69, 75, 78,
        83, 85, 93, 95, 97, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 15, 19, 21, 24, 26, 30, 32, 34, 37, 39, 43, 45, 48, 51, 54, 56, 61, 64, 68, 70, 76, 79,
        86, 89, 95, 96, 97, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 16, 19, 21, 25, 26, 30, 33, 34, 38, 40, 43, 45, 48, 51, 54, 57, 62, 66, 69, 70, 76, 79,
        86, 91, 95, 96, 97, 99, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 16, 19, 21, 25, 26, 31, 33, 35, 38, 40, 43, 45, 49, 51, 55, 57, 63, 66, 70, 72, 77, 81,
        87, 91, 95, 97, 98, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 16, 19, 21, 25, 26, 31, 33, 35, 38, 40, 43, 46, 49, 51, 55, 59, 63, 66, 70, 73, 78, 81,
        88, 91, 95, 97, 98, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 16, 20, 21, 25, 26, 32, 34, 35, 39, 40, 43, 46, 49, 51, 55, 59, 63, 67, 70, 75, 79, 82,
        89, 93, 95, 98, 99, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 16, 20, 21, 25, 27, 32, 34, 35, 39, 41, 44, 47, 49, 52, 56, 60, 64, 67, 71, 76, 80, 83,
        90, 94, 97, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 16, 20, 22, 26, 27, 32, 35, 37, 39, 41, 46, 49, 49, 52, 60, 64, 65, 69, 72, 76, 81, 84,
        90, 94, 97, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 17, 21, 23, 26, 29, 32, 36, 38, 39, 41, 47, 51, 51, 54, 62, 66, 68, 69, 74, 77, 82, 85,
        91, 96, 97, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
];
