package com.code.to.the.moon.model

import software.amazon.awssdk.utils.BinaryUtils


data class UserData (
    var id: Int,
    val firstName: String,
    val lastName: String,
    val age: Int,
    val dietaryRestrictions: DietaryRestrictions,
    val tshirtSize: TshirtSize,
    val profilePicture: ByteArray
) {
    fun fieldMap(): Map<String, String> = mapOf(
        "firstName" to firstName,
        "lastName" to lastName,
        "age" to age.toString(),
        "dietaryRestrictions" to dietaryRestrictions.toString(),
        "tshirtSize" to tshirtSize.toString(),
        "profilePicture" to BinaryUtils.toBase64(profilePicture)
    )
}