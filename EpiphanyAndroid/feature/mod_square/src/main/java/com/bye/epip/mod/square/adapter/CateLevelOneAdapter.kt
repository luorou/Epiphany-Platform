package com.bye.epip.mod.square.adapter

import android.view.LayoutInflater
import android.view.ViewGroup
import androidx.recyclerview.widget.GridLayoutManager
import androidx.recyclerview.widget.RecyclerView
import com.bye.epip.mod.square.beans.CategoryBean
import com.bye.epip.mod.square.databinding.SquareCateLevelOneAdapterBinding

class CateLevelOneAdapter():RecyclerView.Adapter<CateLevelOneAdapter.ViewHolder>() {

    private val mDataResource = ArrayList<CategoryBean>()


    fun setDataS(ds:ArrayList<CategoryBean>){
        this.mDataResource.addAll(ds)
        this.notifyDataSetChanged()
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ViewHolder {
        val b = SquareCateLevelOneAdapterBinding.inflate(LayoutInflater.from(parent.context),parent,false)
        return ViewHolder(b)
    }

    override fun onBindViewHolder(holder: ViewHolder, position: Int) {
        val item = mDataResource[position]
        holder.binding.tvCateName.text = item.name
        val cateAdapter = CateLeveltwoAdapter()
        if (!item.child.isNullOrEmpty()) {
            cateAdapter.setDataS(item.child)
        }
        holder.binding.rv.layoutManager = GridLayoutManager(holder.binding.rv.context,3)
        holder.binding.rv.adapter = cateAdapter
    }

    override fun getItemCount(): Int {
        return mDataResource.size
    }

    class ViewHolder(val binding: SquareCateLevelOneAdapterBinding) :RecyclerView.ViewHolder(binding.root){}


}