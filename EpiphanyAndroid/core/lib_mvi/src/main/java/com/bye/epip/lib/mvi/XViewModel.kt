package com.bye.epip.lib.mvi

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

abstract class XViewModel<STATE, EFFECT, EVENT>
    : ViewModel(), ViewModelContract<EVENT> {

    /**
     *
     */
    protected val _uiState = MutableStateFlow(getState())

    /**
     *
     */
    val uiState = _uiState.asStateFlow()

    /**
     *
     */
    private val _uiEffects: EffectLiveData<EFFECT> = EffectLiveData()

    /**
     *
     */
    internal fun uiEffects(): EffectLiveData<EFFECT> = _uiEffects

    /**
     *
     */
    private var _uiEffect: EFFECT? = null

    protected var uiEffect: EFFECT
        get() = _uiEffect
            ?: throw UninitializedPropertyAccessException("\"viewEffect\" was queried before it is initialized")
        set(value) {
            viewModelScope.launch {
                delay(1)
                _uiEffect = value
                _uiEffects.value = value!!
            }
        }

    /**
     *
     */
    private fun getState(): STATE {
        return initState()
    }

    abstract fun initState(): STATE



}