package com.example.android.encrypt

import android.util.Log

object NdkMore {

    fun testJNI() {
        ld("===========================>")

        val jniString = ProviderJNI.changeJNIString("I am java")
        ld(jniString ?: "exception")

        val char = ProviderJNI.changeJNIChar('a')
        ld(char.toString())

        val byte:Byte = 1
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


    }


}

fun ld(msg: String) {
    Log.d("chenxh", "Kotlin:==>$msg")
}