use crate::lovr_enum;

lovr_enum!(
    DataType, I8x4, U8x4, Sn8x4, Un8x4, Sn10x3, Un10x3, I16, I16x2, I16x4, U16, U16x2, U16x4,
    Sn16x2, Sn16x4, Un16x2, Un16x4, I32, I32x2, I32x3, I32x4, U32, U32x2, U32x3, U32x4, F16x2,
    F16x4, F32, F32x2, F32x3, F32x4, Mat2, Mat3, Mat4, Index16, Index32
);
