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

    /**
     * sm3 签名
     * @param content String
     * @return String
     */
    external fun sm3(content: String): String

    /**
     * sm4 设置密钥
     * @param time ByteArray
     * @return ByteArray
     */
    external fun setSm4Key(time: ByteArray):Boolean

    /**
     * sm4 加密
     * @param content ByteArray
     * @return ByteArray
     */
    external fun sm4Encrypt(content: ByteArray): ByteArray

    /**
     * sm4 解密
     * @param content ByteArray
     * @return ByteArray
     */
    external fun sm4Decrypt(content: ByteArray): ByteArray


}