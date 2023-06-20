package com.bye.epip.lib.mvi.ui

import android.os.Bundle
import android.view.View
import androidx.viewbinding.ViewBinding
import com.bye.epip.lib.mvi.XViewModel

abstract class MviFragment<STATE, EFFECT, EVENT,
        VM : XViewModel<STATE, EFFECT, EVENT>, VB : ViewBinding>
    : VBFragment<VB>() {

    /**
     *
     */
    abstract val VM: VM

    /**
     *
     */
    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)
        VM.uiEffects().observe(viewLifecycleOwner) {
            renderViewEffect(it)
        }
    }

    abstract fun renderViewEffect(eff: EFFECT)

    override fun onDestroy() {
        super.onDestroy()
        VM.uiEffects().removeObservers(this)
    }
}