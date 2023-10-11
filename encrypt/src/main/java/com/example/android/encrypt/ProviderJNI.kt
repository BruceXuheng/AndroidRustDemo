package com.example.android.encrypt

object ProviderJNI {

    init {
        initLogger()
    }

    private external fun initLogger()


    external fun changeJNIString(data: String): String?
    external fun changeJNIChar(data: Char): Char
    external fun changeJNIByte(time: Byte): Byte
    external fun changeJNIInt(time: Int): Int
    external fun changeJNIBoolean(time: Boolean): Boolean
    external fun changeJNILong(time: Long): Long
    external fun changeJNIFloat(time: Float): Float
    external fun changeJNIDouble(time: Double): Double


    external fun changeJNIByteArray(time: ByteArray): ByteArray


}