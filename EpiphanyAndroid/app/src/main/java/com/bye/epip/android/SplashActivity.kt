package com.bye.epip.android

import android.animation.Animator
import android.os.Bundle
import android.view.View
import com.bye.epip.android.databinding.ActivitySplashBinding
import com.bye.epip.lib.mvi.ui.VBActivity
import com.didi.drouter.api.DRouter

class SplashActivity:VBActivity<ActivitySplashBinding>() {
    override fun initVBAndGetRootView(): View {
        this.mBind = ActivitySplashBinding.inflate(layoutInflater)
        return mBind.root
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        mBind.lottieView.addAnimatorListener(object : Animator.AnimatorListener{


            override fun onAnimationStart(p0: Animator) {
            }

            override fun onAnimationEnd(p0: Animator) {
                DRouter.build("/page/index")
                    .start(this@SplashActivity)
            }

            override fun onAnimationCancel(p0: Animator) {
            }

            override fun onAnimationRepeat(p0: Animator) {
            }

        })
    }
}