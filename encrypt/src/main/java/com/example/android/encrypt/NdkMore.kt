package com.example.android.encrypt

import android.util.Log
import java.nio.charset.Charset
import kotlin.system.measureNanoTime


object NdkMore {

    fun testJNI() {
        ld("===========================>")

        val jniString = ProviderJNI.changeJNIString("I am java")
        ld(jniString ?: "exception")

        val char = ProviderJNI.changeJNIChar('a')
        ld(char.toString())

        val byte: Byte = 1
        val byteData = ProviderJNI.changeJNIByte(byte)
        ld(byteData.toString())

        val intData = ProviderJNI.changeJNIInt(0)
        ld(intData.toString())

        val booleanData = ProviderJNI.changeJNIBoolean(true)
        ld(booleanData.toString())

        val nativeTime = ProviderJNI.changeJNILong(111111L)
        ld(nativeTime.toString())

        val nativeFloat = ProviderJNI.changeJNIFloat(111.111f)
        ld(nativeFloat.toString())

        val nativeDouble = ProviderJNI.changeJNIDouble(343.11)
        ld(nativeDouble.toString())


        val stringData = "hello world"
        val nativeByteArray = ProviderJNI.changeJNIByteArray(stringData.toByteArray())
        ld(nativeByteArray.size.toString())
        ld(nativeByteArray.toString(Charset.defaultCharset()))

    }


    fun testSM3() {
        // https://www.oscca.gov.cn/sca/xxgk/2010-12/17/1002389/files/302a3ada057c4a73830536d03e683110.pdf
        var sm3Str: String
        var oneTime = measureNanoTime {
            sm3Str = ProviderJNI.sm3("qwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxzqwewqeqwewqcixzjcxz")

        }
        ld("sm3 time:$oneTime")
        ld("sm3 签名:$sm3Str")

         oneTime = measureNanoTime {
             sm3Str = ProviderJNI.sm3("abcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc")
        }
        ld("sm3 time:$oneTime")
        ld("sm3 签名:$sm3Str")

    }

    fun testSM4() {
        val sm4Key =
            byteArrayOf(102, 1, 34, 12, 98, 45, 12, 11, 5, 123, 22, 107, 2, 16, 12, 43)

        val sm4Status = ProviderJNI.setSm4Key(sm4Key)
        ld("setSm4Key Status = $sm4Status")

        val content = "hello world".toByteArray()

        val sm4Encrypt = ProviderJNI.sm4Encrypt(content)

        val sm4Decrypt = ProviderJNI.sm4Decrypt(sm4Encrypt)

        ld("解密结果:" + String(sm4Decrypt))

    }

}

fun ld(msg: String) {
    Log.d("chenxh", "Kotlin:==>$msg")
}