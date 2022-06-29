<template>
  <div>
    <el-form ref="form" :model="form" label-width="80px" style="padding: 50px">
      <el-form-item label="源格式">
        <el-select v-model="form.inputType" placeholder="请选择源文件类型">
          <el-option label="tsv" value="tsv"></el-option>
          <el-option label="csv" value="csv"></el-option>
          <el-option label="xlsx" value="xlsx"></el-option>
          <el-option label="json" value="json"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="目标格式">
        <el-select v-model="form.outputType" placeholder="请选择目标文件类型">
          <el-option label="tsv" value="tsv"></el-option>
          <el-option label="csv" value="csv"></el-option>
          <el-option label="xlsx" value="xlsx"></el-option>
          <el-option label="json" value="json"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="输入文件">
        <el-input v-model="form.input"></el-input>
      </el-form-item>

      <el-form-item label="输出文件">
        <el-input v-model="form.output"></el-input>
      </el-form-item>
      <el-form-item label="批量处理">
        <el-switch v-model="form.isDir"></el-switch>
      </el-form-item>
      <!-- <el-form-item label="文件">
        <el-upload
          :limit="1"
          action=""
          accept=".tsv,.csv,.xlsx,.json"
          :multiple="false"
          :show-file-list="false"
          :on-error="handleError"
          :on-success="handleSuccess"
          :before-upload="beforeUpload"
          :http-request="handlerRequest"
        >
          <el-button size="small" type="primary">点击选择</el-button>
          <div slot="tip" class="el-upload__tip">仅支持所选文件类型</div>
        </el-upload>
      </el-form-item> -->
      <el-form-item>
        <el-button type="primary" @click="onSubmit">立即转换</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";
export default {
  data() {
    return {
      fileList: [],
      form: {
        inputType: "",
        outputType: "",
        input: "",
        output: "",
        isDir: false,
      },
    };
  },
  methods: {
    onSubmit() {
      if (this.form.input != "" && this.form.output != "") {
        invoke("handler_file", {
          inputType: this.form.inputType,
          outputType: this.form.outputType,
          input: this.form.input,
          output: this.form.output,
          isDir: this.form.isDir
        })
          .then((result) => {
            this.$message.success("转换完成");
          })
          .catch((error) => {
            this.$message.error(JSON.stringify(error));
          });
      }
    },
    handlerRequest(options) {
      // this.$message.success("上传中");
      let file = options.file;
      let fileBlobName = null;
      if (window.createObjectURL !== undefined) {
        fileBlobName = window.createObjectURL(file);
      } else if (window.URL !== undefined) {
        fileBlobName = window.URL.createObjectURL(file);
      } else if (window.webkitURL !== undefined) {
        fileBlobName = window.webkitURL.createObjectURL(file);
      } else {
        this.$message.error("浏览器不支持上传文件");
        return false;
      }
      this.form.file = fileBlobName;
      // console.log("fileBlobName", fileBlobName);
      // console.log("options", options);
    },
    handleError() {
      this.$message.error("上传失败");
    },
    handleSuccess(obj) {
      // console.log("上传成功", obj);
      // 上传完毕，在这里进行保存
    },
    beforeUpload(file, fileList) {
      // 1.校验文件,分为
      // 2.获取远程的fileKey
      // this.$message.success("上传前");
    },
  },
};
</script>

</style>