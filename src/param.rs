pub static BEST_N: [[usize; 41]; 91] = [
    [
        5, 6, 7, 7, 7, 7, 7, 9, 10, 10, 10, 10, 9, 11, 13, 14, 15, 15, 14, 15, 16, 19, 21, 22, 22,
        25, 27, 28, 28, 31, 34, 36, 37, 45, 53, 57, 61, 62, 62, 74, 86,
    ],
    [
        5, 7, 8, 8, 8, 8, 8, 9, 9, 10, 11, 11, 11, 13, 15, 16, 16, 16, 16, 17, 18, 20, 21, 21, 21,
        25, 28, 29, 30, 33, 36, 39, 41, 46, 51, 56, 61, 68, 74, 80, 86,
    ],
    [
        5, 7, 8, 8, 8, 9, 9, 9, 8, 10, 12, 12, 12, 14, 16, 16, 16, 17, 18, 19, 20, 21, 21, 21, 20,
        25, 29, 31, 32, 35, 37, 41, 45, 47, 48, 55, 61, 73, 85, 85, 85,
    ],
    [
        5, 7, 8, 9, 9, 10, 10, 11, 11, 12, 13, 13, 13, 15, 16, 17, 17, 18, 19, 20, 21, 23, 24, 24,
        24, 29, 34, 35, 36, 38, 40, 45, 50, 51, 51, 61, 71, 78, 85, 85, 85,
    ],
    [
        5, 7, 8, 9, 9, 10, 10, 12, 13, 13, 13, 14, 14, 15, 15, 16, 17, 18, 19, 20, 21, 24, 27, 27,
        27, 33, 39, 39, 39, 41, 42, 49, 55, 54, 53, 67, 81, 83, 85, 85, 85,
    ],
    [
        5, 7, 8, 9, 9, 10, 11, 13, 14, 14, 14, 14, 14, 16, 17, 18, 18, 19, 20, 21, 22, 25, 27, 28,
        28, 33, 37, 39, 41, 42, 43, 49, 54, 57, 59, 69, 79, 82, 84, 86, 88,
    ],
    [
        5, 6, 7, 8, 9, 11, 12, 13, 14, 14, 14, 14, 14, 16, 18, 19, 19, 20, 21, 22, 22, 25, 27, 28,
        29, 32, 35, 39, 43, 44, 44, 49, 53, 59, 65, 71, 77, 80, 82, 86, 90,
    ],
    [
        5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 15, 15, 15, 17, 19, 20, 20, 21, 21, 23, 24, 25, 26, 29,
        31, 33, 35, 40, 44, 45, 46, 50, 53, 62, 70, 74, 78, 81, 84, 86, 88,
    ],
    [
        5, 7, 8, 9, 10, 11, 12, 13, 13, 14, 15, 16, 16, 18, 19, 20, 21, 21, 21, 24, 26, 26, 25, 29,
        32, 33, 34, 40, 45, 47, 48, 51, 53, 64, 74, 77, 79, 82, 85, 86, 86,
    ],
    [
        5, 7, 9, 10, 10, 11, 12, 13, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 22, 25, 27, 27, 27,
        31, 34, 35, 36, 43, 49, 50, 51, 53, 55, 64, 73, 77, 81, 82, 83, 88, 93,
    ],
    [
        5, 7, 9, 9, 9, 11, 12, 13, 13, 14, 15, 17, 18, 19, 19, 20, 21, 22, 23, 25, 27, 28, 29, 33,
        36, 37, 38, 46, 53, 53, 53, 55, 57, 65, 72, 78, 83, 82, 80, 90, 100,
    ],
    [
        6, 8, 9, 10, 10, 12, 13, 14, 14, 16, 17, 18, 18, 20, 21, 21, 21, 23, 25, 26, 26, 29, 31,
        34, 36, 37, 38, 44, 50, 52, 54, 57, 60, 65, 70, 74, 78, 83, 88, 91, 94,
    ],
    [
        6, 7, 8, 9, 10, 12, 14, 15, 15, 17, 18, 18, 18, 20, 22, 22, 21, 24, 27, 26, 24, 29, 33, 34,
        35, 36, 37, 42, 47, 51, 54, 58, 62, 65, 68, 71, 73, 85, 96, 92, 87,
    ],
    [
        6, 8, 9, 10, 11, 13, 14, 15, 16, 17, 18, 19, 19, 21, 23, 23, 23, 26, 28, 27, 25, 29, 33,
        36, 38, 40, 41, 45, 48, 51, 54, 59, 63, 67, 71, 76, 80, 88, 95, 95, 94,
    ],
    [
        6, 8, 10, 11, 12, 13, 13, 15, 17, 18, 18, 19, 20, 22, 23, 24, 25, 27, 28, 27, 26, 29, 32,
        37, 41, 43, 45, 47, 48, 51, 54, 59, 63, 69, 74, 80, 86, 90, 93, 97, 100,
    ],
    [
        6, 9, 11, 12, 13, 13, 13, 15, 17, 18, 18, 20, 21, 22, 22, 24, 26, 27, 27, 28, 28, 32, 35,
        36, 37, 41, 45, 47, 49, 53, 57, 60, 63, 72, 80, 84, 88, 90, 92, 96, 100,
    ],
    [
        6, 9, 11, 12, 13, 13, 13, 15, 16, 17, 18, 20, 21, 21, 21, 24, 26, 26, 26, 28, 30, 34, 38,
        36, 33, 39, 45, 48, 50, 55, 59, 61, 63, 74, 85, 88, 90, 90, 90, 95, 99,
    ],
    [
        6, 9, 11, 12, 13, 13, 13, 15, 17, 18, 18, 20, 21, 22, 22, 23, 24, 25, 26, 29, 31, 34, 36,
        36, 36, 39, 41, 46, 50, 53, 56, 63, 69, 76, 82, 86, 89, 92, 94, 97, 100,
    ],
    [
        6, 9, 11, 12, 12, 12, 12, 15, 17, 18, 18, 20, 21, 22, 23, 23, 22, 24, 26, 29, 31, 32, 33,
        36, 38, 38, 37, 43, 49, 51, 53, 64, 74, 77, 79, 83, 87, 93, 98, 99, 100,
    ],
    [
        6, 8, 10, 11, 12, 13, 14, 16, 17, 18, 19, 21, 22, 23, 24, 24, 24, 26, 28, 29, 30, 34, 37,
        38, 39, 41, 43, 46, 48, 53, 58, 65, 72, 76, 80, 86, 91, 93, 94, 97, 99,
    ],
    [
        6, 8, 9, 10, 11, 14, 16, 16, 16, 18, 20, 21, 22, 23, 24, 25, 26, 28, 30, 30, 29, 35, 40,
        40, 40, 44, 48, 48, 47, 55, 62, 66, 70, 76, 81, 88, 94, 92, 90, 94, 98,
    ],
    [
        6, 8, 10, 11, 12, 14, 16, 17, 17, 19, 20, 21, 22, 23, 24, 26, 28, 28, 28, 30, 32, 36, 40,
        41, 41, 45, 49, 50, 51, 56, 60, 67, 73, 78, 82, 89, 95, 95, 94, 96, 97,
    ],
    [
        6, 9, 11, 12, 12, 14, 15, 17, 18, 19, 19, 21, 22, 23, 23, 26, 29, 27, 25, 30, 35, 37, 39,
        40, 41, 46, 50, 53, 55, 57, 58, 67, 75, 79, 82, 89, 96, 97, 98, 97, 95,
    ],
    [
        7, 10, 12, 12, 12, 14, 15, 18, 20, 19, 18, 21, 23, 24, 25, 28, 30, 30, 29, 33, 36, 38, 39,
        41, 43, 46, 48, 53, 57, 59, 61, 67, 72, 78, 84, 90, 96, 98, 99, 96, 93,
    ],
    [
        7, 10, 12, 12, 12, 14, 15, 18, 21, 19, 17, 21, 24, 26, 27, 29, 31, 32, 33, 35, 36, 38, 39,
        42, 45, 45, 45, 52, 58, 61, 64, 66, 68, 77, 85, 90, 95, 98, 100, 96, 91,
    ],
    [
        7, 10, 12, 13, 13, 15, 16, 18, 20, 20, 20, 23, 25, 25, 25, 28, 30, 32, 33, 35, 37, 38, 38,
        42, 45, 46, 47, 53, 58, 63, 67, 71, 75, 82, 88, 92, 96, 98, 100, 97, 94,
    ],
    [
        7, 9, 11, 13, 14, 15, 16, 17, 18, 20, 22, 24, 25, 24, 23, 26, 28, 30, 32, 35, 37, 37, 36,
        41, 45, 47, 49, 54, 58, 64, 70, 76, 82, 87, 91, 94, 97, 99, 100, 99, 97,
    ],
    [
        7, 9, 11, 13, 14, 15, 16, 17, 18, 20, 22, 24, 25, 25, 25, 27, 29, 31, 32, 35, 38, 38, 38,
        43, 48, 49, 50, 57, 63, 67, 70, 77, 84, 88, 91, 93, 95, 96, 96, 97, 98,
    ],
    [
        7, 9, 11, 13, 14, 15, 16, 17, 18, 20, 22, 24, 25, 26, 26, 28, 29, 30, 31, 35, 38, 39, 40,
        45, 50, 50, 50, 59, 68, 69, 69, 77, 85, 88, 90, 92, 93, 93, 92, 96, 99,
    ],
    [
        7, 10, 12, 14, 15, 16, 16, 18, 19, 20, 21, 23, 25, 26, 27, 29, 31, 32, 32, 35, 37, 39, 41,
        46, 50, 52, 53, 59, 64, 68, 71, 78, 85, 87, 88, 92, 96, 93, 89, 94, 99,
    ],
    [
        7, 10, 12, 14, 15, 15, 15, 17, 19, 20, 20, 23, 25, 27, 28, 30, 32, 33, 33, 34, 35, 38, 41,
        46, 50, 53, 55, 58, 60, 66, 72, 79, 85, 86, 86, 92, 98, 92, 86, 92, 98,
    ],
    [
        7, 10, 12, 14, 15, 15, 15, 18, 20, 21, 22, 24, 26, 28, 29, 30, 30, 32, 34, 36, 37, 40, 43,
        47, 50, 52, 54, 58, 62, 67, 71, 76, 81, 86, 90, 93, 96, 95, 93, 95, 96,
    ],
    [
        7, 10, 12, 14, 15, 15, 15, 18, 20, 22, 23, 25, 26, 28, 29, 28, 27, 31, 35, 37, 39, 42, 44,
        47, 50, 51, 52, 58, 63, 67, 70, 73, 76, 85, 94, 94, 94, 97, 100, 97, 93,
    ],
    [
        7, 10, 13, 14, 15, 17, 18, 19, 20, 21, 22, 25, 28, 29, 30, 30, 29, 32, 35, 38, 41, 43, 45,
        47, 48, 52, 56, 61, 66, 71, 75, 77, 79, 88, 97, 96, 95, 98, 100, 98, 95,
    ],
    [
        7, 11, 14, 15, 15, 18, 20, 20, 20, 21, 21, 25, 29, 30, 30, 30, 30, 32, 34, 38, 42, 44, 45,
        46, 46, 53, 60, 65, 69, 75, 80, 81, 81, 90, 99, 97, 95, 98, 100, 98, 96,
    ],
    [
        7, 11, 14, 15, 16, 18, 19, 20, 21, 22, 22, 26, 29, 31, 32, 31, 30, 34, 37, 40, 43, 44, 45,
        48, 51, 55, 58, 62, 65, 72, 78, 82, 85, 90, 95, 96, 97, 98, 99, 98, 97,
    ],
    [
        7, 11, 14, 15, 16, 17, 18, 20, 21, 22, 22, 26, 29, 31, 33, 32, 30, 35, 40, 42, 43, 44, 45,
        50, 55, 55, 55, 58, 61, 68, 75, 82, 89, 90, 90, 95, 99, 99, 98, 98, 97,
    ],
    [
        7, 11, 14, 15, 16, 17, 18, 20, 21, 23, 24, 27, 29, 30, 31, 32, 32, 36, 40, 40, 40, 43, 46,
        50, 53, 55, 57, 61, 64, 72, 80, 83, 86, 91, 95, 98, 100, 99, 97, 98, 99,
    ],
    [
        7, 11, 14, 15, 15, 17, 18, 20, 21, 24, 26, 27, 28, 28, 28, 31, 33, 37, 40, 38, 36, 42, 47,
        49, 50, 55, 59, 63, 67, 76, 85, 84, 83, 92, 100, 100, 100, 98, 96, 98, 100,
    ],
    [
        8, 11, 13, 14, 15, 17, 18, 21, 23, 24, 25, 27, 28, 30, 31, 33, 35, 38, 40, 40, 40, 45, 49,
        52, 55, 57, 59, 63, 66, 75, 83, 84, 85, 93, 100, 98, 95, 97, 98, 97, 96,
    ],
    [
        8, 10, 12, 14, 15, 17, 18, 21, 24, 24, 23, 26, 28, 31, 33, 35, 37, 38, 39, 42, 44, 48, 51,
        56, 60, 59, 58, 62, 65, 73, 81, 84, 87, 93, 99, 95, 90, 95, 99, 96, 92,
    ],
    [
        8, 11, 13, 15, 16, 18, 19, 21, 23, 24, 24, 27, 29, 31, 32, 34, 36, 38, 40, 42, 43, 47, 51,
        55, 59, 62, 65, 67, 68, 77, 86, 90, 93, 95, 96, 94, 91, 96, 100, 98, 96,
    ],
    [
        8, 11, 13, 15, 17, 18, 19, 21, 22, 24, 25, 28, 30, 30, 30, 33, 35, 38, 40, 41, 41, 46, 50,
        54, 58, 65, 72, 71, 70, 80, 90, 94, 98, 95, 92, 92, 92, 96, 100, 100, 99,
    ],
    [
        8, 11, 13, 16, 18, 19, 19, 21, 23, 25, 27, 28, 29, 31, 33, 34, 35, 39, 42, 43, 44, 49, 53,
        55, 57, 63, 69, 70, 71, 80, 89, 92, 95, 93, 90, 93, 95, 98, 100, 100, 100,
    ],
    [
        8, 11, 13, 16, 18, 19, 19, 21, 23, 26, 28, 28, 27, 31, 35, 35, 35, 39, 43, 45, 46, 51, 55,
        55, 55, 60, 65, 68, 71, 79, 87, 90, 92, 90, 88, 93, 98, 99, 99, 100, 100,
    ],
    [
        8, 11, 14, 17, 19, 19, 19, 21, 23, 26, 28, 29, 29, 32, 35, 37, 38, 41, 43, 45, 46, 50, 53,
        55, 57, 62, 67, 70, 73, 80, 86, 90, 94, 93, 92, 95, 98, 99, 99, 100, 100,
    ],
    [
        8, 12, 15, 17, 19, 19, 19, 21, 23, 25, 27, 29, 30, 32, 34, 37, 40, 42, 43, 44, 45, 48, 51,
        55, 59, 64, 69, 72, 74, 80, 85, 90, 95, 96, 96, 97, 97, 98, 98, 99, 99,
    ],
    [
        8, 12, 15, 17, 19, 20, 20, 22, 23, 26, 28, 29, 29, 33, 36, 39, 41, 42, 43, 44, 45, 49, 53,
        58, 63, 65, 66, 72, 78, 81, 84, 90, 95, 97, 98, 98, 97, 98, 99, 100, 100,
    ],
    [
        8, 12, 15, 17, 19, 20, 21, 22, 23, 26, 29, 29, 28, 33, 37, 39, 41, 42, 42, 43, 44, 49, 54,
        61, 67, 65, 63, 73, 82, 83, 83, 89, 95, 97, 99, 98, 97, 98, 99, 100, 100,
    ],
    [
        8, 12, 15, 17, 18, 20, 22, 23, 24, 27, 29, 29, 28, 32, 36, 38, 40, 42, 43, 45, 46, 50, 53,
        61, 68, 67, 66, 75, 83, 84, 85, 90, 94, 97, 99, 98, 96, 97, 97, 99, 100,
    ],
    [
        8, 12, 15, 16, 16, 20, 23, 24, 24, 26, 28, 28, 28, 32, 35, 37, 39, 41, 43, 46, 48, 50, 52,
        60, 68, 69, 69, 77, 84, 85, 86, 89, 92, 96, 99, 97, 95, 95, 95, 97, 99,
    ],
    [
        9, 12, 15, 16, 17, 20, 23, 24, 24, 26, 27, 30, 32, 35, 37, 39, 40, 42, 44, 46, 48, 51, 54,
        59, 63, 64, 65, 75, 84, 86, 87, 92, 96, 98, 100, 99, 98, 98, 98, 99, 99,
    ],
    [
        9, 12, 15, 16, 17, 20, 22, 23, 23, 25, 26, 31, 35, 37, 38, 40, 41, 43, 45, 47, 48, 52, 56,
        57, 58, 60, 61, 72, 83, 86, 88, 94, 100, 100, 100, 100, 100, 100, 100, 100, 99,
    ],
    [
        9, 13, 16, 17, 17, 20, 22, 23, 24, 26, 28, 32, 35, 35, 34, 37, 40, 42, 44, 47, 49, 53, 56,
        58, 60, 65, 69, 77, 84, 86, 87, 93, 99, 97, 95, 95, 95, 97, 99, 96, 92,
    ],
    [
        9, 13, 16, 17, 17, 20, 22, 23, 24, 27, 30, 33, 35, 33, 30, 34, 38, 41, 43, 46, 49, 52, 55,
        58, 61, 69, 77, 81, 85, 86, 86, 92, 97, 94, 90, 90, 90, 94, 98, 92, 85,
    ],
    [
        9, 13, 16, 17, 18, 20, 22, 22, 22, 26, 30, 32, 33, 33, 33, 37, 40, 43, 45, 47, 49, 54, 58,
        60, 61, 69, 76, 79, 82, 84, 86, 92, 98, 97, 95, 94, 92, 95, 98, 96, 93,
    ],
    [
        8, 12, 16, 17, 18, 20, 22, 21, 20, 25, 29, 30, 30, 33, 35, 39, 42, 45, 47, 48, 48, 55, 61,
        61, 61, 68, 74, 76, 78, 82, 85, 92, 99, 99, 99, 97, 94, 96, 98, 99, 100,
    ],
    [
        9, 13, 16, 17, 18, 21, 23, 24, 24, 27, 30, 31, 32, 34, 35, 38, 40, 44, 48, 50, 51, 56, 61,
        63, 65, 73, 80, 81, 82, 84, 86, 90, 94, 96, 98, 98, 97, 98, 98, 99, 100,
    ],
    [
        9, 12, 15, 16, 17, 21, 24, 26, 28, 30, 31, 32, 33, 34, 35, 37, 38, 44, 49, 52, 54, 58, 61,
        65, 69, 77, 85, 85, 85, 86, 86, 87, 88, 92, 96, 98, 100, 99, 97, 99, 100,
    ],
    [
        10, 13, 15, 16, 17, 21, 24, 26, 28, 29, 30, 32, 34, 34, 34, 37, 40, 45, 49, 52, 55, 59, 63,
        66, 69, 77, 84, 85, 86, 88, 89, 91, 92, 95, 98, 99, 99, 99, 98, 99, 99,
    ],
    [
        10, 13, 15, 16, 17, 21, 24, 26, 28, 29, 29, 32, 35, 34, 33, 37, 41, 45, 49, 52, 55, 60, 64,
        66, 68, 76, 83, 85, 86, 89, 92, 94, 96, 98, 100, 99, 97, 98, 98, 98, 97,
    ],
    [
        10, 13, 15, 17, 18, 22, 25, 27, 28, 29, 30, 32, 33, 34, 35, 39, 42, 45, 48, 51, 54, 57, 60,
        65, 69, 76, 83, 86, 89, 90, 91, 95, 98, 99, 100, 99, 97, 98, 99, 98, 97,
    ],
    [
        9, 12, 15, 17, 18, 22, 25, 27, 28, 29, 30, 30, 30, 33, 36, 40, 43, 45, 47, 50, 52, 54, 55,
        62, 69, 76, 82, 87, 91, 90, 89, 94, 99, 99, 99, 98, 96, 98, 99, 98, 97,
    ],
    [
        9, 13, 17, 18, 18, 22, 25, 27, 28, 30, 32, 32, 32, 35, 38, 42, 45, 47, 48, 51, 54, 57, 59,
        65, 70, 77, 84, 86, 88, 90, 92, 96, 100, 100, 100, 99, 98, 99, 100, 100, 99,
    ],
    [
        9, 14, 18, 18, 18, 22, 25, 27, 28, 31, 34, 34, 34, 37, 40, 44, 47, 48, 49, 52, 55, 59, 62,
        66, 70, 78, 85, 85, 85, 90, 94, 97, 100, 100, 100, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 14, 17, 18, 19, 22, 24, 27, 30, 32, 33, 34, 34, 37, 40, 44, 47, 48, 49, 53, 57, 60, 63,
        67, 70, 77, 83, 86, 88, 91, 94, 97, 99, 100, 100, 99, 98, 99, 99, 100, 100,
    ],
    [
        10, 13, 16, 18, 20, 22, 23, 27, 31, 32, 32, 33, 33, 36, 39, 43, 47, 48, 48, 54, 59, 61, 63,
        67, 70, 76, 81, 86, 90, 92, 94, 96, 97, 99, 100, 98, 96, 97, 97, 99, 100,
    ],
    [
        10, 14, 17, 19, 20, 23, 25, 27, 29, 31, 33, 34, 34, 37, 40, 43, 46, 47, 48, 53, 57, 61, 65,
        70, 74, 78, 81, 85, 88, 90, 92, 94, 96, 98, 100, 98, 96, 98, 99, 100, 100,
    ],
    [
        9, 13, 17, 19, 20, 23, 26, 26, 26, 30, 34, 35, 35, 38, 40, 43, 45, 47, 48, 51, 54, 61, 67,
        73, 78, 79, 80, 83, 85, 87, 89, 92, 95, 98, 100, 98, 95, 98, 100, 100, 100,
    ],
    [
        9, 13, 17, 19, 21, 24, 26, 27, 28, 32, 35, 35, 35, 39, 42, 44, 46, 47, 48, 53, 58, 64, 69,
        74, 78, 80, 82, 84, 85, 88, 91, 93, 94, 97, 99, 98, 97, 99, 100, 100, 99,
    ],
    [
        9, 13, 17, 19, 21, 24, 26, 28, 29, 33, 36, 35, 34, 39, 43, 45, 46, 47, 48, 55, 62, 67, 71,
        74, 77, 80, 83, 84, 84, 88, 92, 92, 92, 95, 98, 98, 98, 99, 100, 99, 97,
    ],
    [
        10, 13, 16, 18, 20, 22, 24, 27, 30, 32, 34, 35, 35, 40, 45, 46, 46, 48, 50, 56, 62, 67, 72,
        75, 78, 80, 82, 85, 87, 92, 96, 95, 94, 97, 99, 99, 99, 100, 100, 100, 99,
    ],
    [
        10, 13, 15, 17, 19, 21, 22, 27, 31, 32, 32, 34, 35, 41, 46, 46, 45, 48, 51, 56, 61, 67, 72,
        76, 79, 80, 80, 85, 89, 95, 100, 98, 95, 97, 99, 100, 100, 100, 100, 100, 100,
    ],
    [
        10, 13, 15, 18, 20, 23, 25, 28, 30, 32, 34, 35, 35, 40, 45, 46, 46, 49, 52, 55, 58, 63, 67,
        73, 78, 81, 83, 88, 93, 95, 97, 98, 98, 99, 100, 100, 100, 100, 100, 100, 99,
    ],
    [
        10, 13, 15, 18, 21, 24, 27, 28, 29, 33, 36, 36, 35, 39, 43, 45, 46, 50, 53, 54, 55, 59, 62,
        70, 77, 82, 86, 91, 96, 95, 94, 97, 100, 100, 100, 100, 99, 100, 100, 99, 97,
    ],
    [
        10, 13, 16, 19, 21, 24, 27, 28, 28, 32, 36, 36, 36, 40, 43, 45, 47, 50, 53, 56, 59, 64, 68,
        73, 78, 82, 86, 91, 96, 95, 94, 96, 98, 99, 100, 99, 98, 99, 99, 99, 98,
    ],
    [
        9, 13, 17, 19, 21, 24, 27, 27, 27, 31, 35, 36, 36, 39, 42, 45, 48, 51, 53, 58, 63, 69, 74,
        76, 78, 82, 86, 91, 95, 94, 93, 94, 95, 97, 99, 98, 96, 97, 98, 98, 98,
    ],
    [
        10, 14, 17, 19, 21, 23, 25, 27, 29, 33, 36, 36, 36, 40, 43, 46, 48, 51, 54, 60, 65, 69, 73,
        78, 82, 85, 88, 91, 93, 95, 97, 97, 97, 99, 100, 99, 98, 98, 98, 99, 99,
    ],
    [
        10, 14, 17, 19, 21, 22, 23, 27, 31, 34, 36, 36, 35, 40, 44, 46, 48, 51, 54, 60, 66, 69, 71,
        78, 85, 87, 89, 90, 90, 95, 100, 99, 98, 99, 100, 100, 100, 99, 97, 98, 99,
    ],
    [
        10, 14, 17, 20, 22, 24, 26, 29, 31, 34, 36, 37, 38, 41, 44, 47, 50, 53, 56, 61, 66, 67, 68,
        75, 81, 83, 84, 87, 90, 93, 96, 97, 97, 98, 99, 99, 99, 99, 99, 100, 100,
    ],
    [
        10, 13, 16, 20, 23, 26, 28, 29, 30, 33, 35, 38, 40, 42, 44, 48, 51, 54, 57, 61, 65, 65, 65,
        71, 77, 78, 79, 84, 89, 91, 92, 94, 96, 97, 98, 98, 98, 99, 100, 100, 100,
    ],
    [
        10, 13, 16, 19, 22, 25, 27, 29, 30, 32, 34, 38, 41, 43, 44, 47, 49, 54, 59, 62, 64, 68, 72,
        77, 81, 82, 83, 86, 88, 90, 92, 95, 98, 98, 98, 98, 97, 99, 100, 100, 100,
    ],
    [
        10, 13, 16, 19, 21, 23, 25, 28, 30, 32, 33, 38, 42, 43, 44, 46, 47, 54, 61, 62, 63, 71, 79,
        82, 85, 86, 86, 86, 86, 89, 91, 96, 100, 99, 97, 96, 95, 97, 99, 99, 99,
    ],
    [
        10, 14, 17, 20, 22, 26, 29, 30, 31, 33, 34, 38, 42, 43, 43, 46, 49, 54, 58, 62, 65, 71, 77,
        81, 84, 86, 87, 90, 92, 93, 93, 97, 100, 99, 97, 98, 98, 99, 100, 99, 98,
    ],
    [
        10, 14, 17, 20, 22, 27, 32, 32, 31, 33, 34, 38, 41, 41, 41, 46, 51, 53, 54, 60, 66, 71, 75,
        79, 82, 85, 88, 93, 98, 96, 94, 97, 100, 98, 96, 98, 100, 100, 100, 98, 96,
    ],
    [
        10, 14, 18, 20, 22, 26, 30, 31, 32, 34, 35, 38, 40, 43, 45, 48, 51, 54, 57, 63, 68, 73, 77,
        81, 85, 88, 90, 94, 98, 96, 93, 95, 97, 98, 98, 99, 99, 99, 99, 99, 98,
    ],
    [
        10, 15, 19, 21, 22, 25, 27, 30, 33, 35, 36, 37, 38, 44, 49, 50, 51, 55, 59, 64, 69, 74, 78,
        83, 88, 90, 92, 95, 97, 95, 92, 93, 94, 97, 100, 99, 97, 98, 98, 99, 99,
    ],
    [
        10, 14, 18, 20, 22, 24, 26, 29, 32, 35, 37, 39, 40, 44, 47, 49, 51, 56, 60, 65, 69, 73, 77,
        83, 88, 90, 91, 95, 98, 97, 95, 96, 96, 98, 100, 100, 99, 97, 95, 98, 100,
    ],
    [
        10, 14, 17, 19, 21, 23, 25, 28, 31, 35, 38, 40, 42, 43, 44, 47, 50, 56, 61, 65, 68, 72, 76,
        82, 87, 88, 89, 94, 99, 98, 97, 97, 97, 99, 100, 100, 100, 96, 92, 96, 100,
    ],
    [
        10, 14, 18, 21, 24, 26, 27, 30, 32, 36, 39, 42, 44, 44, 43, 48, 52, 56, 60, 64, 68, 73, 78,
        82, 86, 88, 89, 93, 96, 98, 99, 99, 98, 99, 100, 100, 100, 97, 94, 97, 100,
    ],
    [
        10, 15, 19, 23, 26, 27, 28, 30, 32, 36, 40, 43, 45, 44, 42, 48, 54, 57, 59, 64, 68, 74, 79,
        82, 85, 87, 89, 91, 93, 97, 100, 99, 98, 99, 100, 100, 100, 98, 96, 98, 100,
    ],
];
