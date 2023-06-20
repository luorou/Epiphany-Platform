package com.bye.epip.mod.index.ui

import android.animation.Animator
import android.os.Bundle
import android.view.View
import com.bye.epip.lib.mvi.ui.VBActivity
import com.bye.epip.mod.index.databinding.IndexSplashActivityBinding
import com.didi.drouter.api.DRouter

class WelcomeActivity:VBActivity<IndexSplashActivityBinding>() {
    override fun initVBAndGetRootView(): View {
        this.mBind = IndexSplashActivityBinding.inflate(layoutInflater)
        return mBind.root
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        mBind.lottieView.addAnimatorListener(object : Animator.AnimatorListener{


            override fun onAnimationStart(p0: Animator) {

            }

            override fun onAnimationEnd(p0: Animator) {
                DRouter.build("/page/index")
                    .start(this@WelcomeActivity)
            }

            override fun onAnimationCancel(p0: Animator) {

            }

            override fun onAnimationRepeat(p0: Animator) {

            }

        })
    }
}