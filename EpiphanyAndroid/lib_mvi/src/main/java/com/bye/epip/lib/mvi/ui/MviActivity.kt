package com.bye.epip.lib.mvi.ui

import android.os.Bundle
import androidx.viewbinding.ViewBinding
import com.bye.epip.lib.mvi.XViewModel

abstract class MviActivity<STATE, EFFECT, EVENT,
        VM : XViewModel<STATE, EFFECT, EVENT>, VB : ViewBinding>
    : VBActivity<VB>() {

    abstract val VM: VM

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        //
        VM.uiEffects().observe(this) {
            renderViewEffect(it)
        }
    }

    abstract fun renderViewEffect(eff: EFFECT)

    override fun onDestroy() {
        super.onDestroy()
        VM.uiEffects().removeObservers(this)
    }
}