package com.bye.epip.lib.mvi

internal interface ViewModelContract<T> {
    fun process(ev: T)
}