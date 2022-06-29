<template>
    <div>
        <el-card class="box-card">
            <el-form ref="form" :model="form" label-width="100px">
                <el-form-item label="本地工具地址">
                    <el-input v-model="form.server"></el-input>
                </el-form-item>
               <!-- <el-form-item label="本地监听文件">
                    <el-input v-model="form.listen"></el-input>
                </el-form-item> -->
                <el-form-item>
                    <el-button type="primary" @click="onSubmit">立即保存</el-button>
                </el-form-item>
            </el-form>
        </el-card>

    </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri'
export default {
    name: "Settings",
    data() {
        return {
            form: {
                server: '',
                listen:'',
            }
        }
    },
    mounted(){
        this.getConfig();
    },
    methods: {
        onSubmit() {
            this.setConfig();
            console.log('submit!');
        },
        getConfig(){
            let _that = this;
            invoke('get_config_by_file',{}).then(result=>{
                //   _that.$message.success(JSON.stringify(result))
                  _that.form.server = result.server;
                  _that.form.listen = result.listen;
            }).catch(error=>{
                 this.$message.error(JSON.stringify(error))
            });
         
        },
        setConfig(){
            let server = this.form.server;
            let listen = this.form.listen;
            // this.$message.error(JSON.stringify(this.form))
            if (server){
                invoke('save_config_by_file',{config:{listen:listen,server:server}}).then(result=>{
                    this.$message.success("保存成功")
                }).catch(error=>{
                    this.$message.error(JSON.stringify(error))
                });
            }else{
                 this.$message.error("请填写完整信息")
            }
        }
    }
}
</script>

<style>
</style>