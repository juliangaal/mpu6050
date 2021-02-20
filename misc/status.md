|in Register Map|API|Register|Access|Bits|
|:-----|:---:|:------:|:-----:|---:|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x00] AUX_VDDIO|	R/W |	[7] AUX_VDDIO	[6:1] XG_OFFS_TC	[0] OTP_BNK_VLD| 
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x01] YG_OFFS_TC|	R/W |	 	[6:1] YG_OFFS_TC|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x02] ZG_OFFS_TC|	R/W |	 	[6:1] ZG_OFFS_TC|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x03] X_FINE_GAIN|	R/W |	[7:0] X_FINE_GAIN|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x04] Y_FINE_GAIN|	R/W |	[7:0] Y_FINE_GAIN|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x05] Z_FINE_GAIN|	R/W |	[7:0] Z_FINE_GAIN|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x06] XA_OFFS_H|	R/W |	[15:0] XA_OFFS|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x07] XA_OFFS_L_TC|	R/W ||
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x08] YA_OFFS_H|	R/W |	[15:0] YA_OFFS|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x09] YA_OFFS_L_TC|	R/W ||
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x0A] ZA_OFFS_H|	R/W |	[15:0] ZA_OFFS|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x0B] ZA_OFFS_L_TC|	R/W ||
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x13] XG_OFFS_USRH|	R/W |	[15:0] XG_OFFS_USR|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x14] XG_OFFS_USRL|	R/W ||
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x15] YG_OFFS_USRH|	R/W |	[15:0] YG_OFFS_USR|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x16] YG_OFFS_USRL|	R/W ||
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x17] ZG_OFFS_USRH|	R/W |	[15:0] ZG_OFFS_USR|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x18] ZG_OFFS_USRL|	R/W ||
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x19] SMPLRT_DIV|	R/W |	[7:0] SMPLRT_DIV|
| <ul><li> -[x] </li></ul>|<ul><li> -[ ] </li></ul>|[0x1A] CONFIG|	R/W |	 	[5:3] EXT_SYNC_SET	[2:0] DLPF_CFG|
| <ul><li> -[x] </li></ul>|<ul><li> -[x] </li></ul>|[0x1B] GYRO_CONFIG|	R/W |	[7] XG_ST	[6] YG_ST	[5] ZG_ST	[4:3] FS_SEL|	 
| <ul><li> -[x] </li></ul>|<ul><li> -[x] </li></ul>|[0x1C] ACCEL_CONFIG|	R/W |	[7] XA_ST	[6] YA_ST	[5] ZA_ST	[4:3] AFS_SEL	[2:0] ACCEL_HPF|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x1D] FF_THR|	R/W |	[7:0] FF_THR|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x1E] FF_DUR|	R/W |	[7:0] FF_DUR|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x1F] MOT_THR|	R/W |	[7:0] MOT_THR|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x20] MOT_DUR|	R/W |	[7:0] MOT_DUR|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x21] ZRMOT_THR|	R/W |	[7:0] ZRMOT_THR|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x22] ZRMOT_DUR|	R/W |	[7:0] ZRMOT_DUR|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x23] FIFO_EN|	R/W |	[7] TEMP_FIFO_EN	[6] XG_FIFO_EN	[5] YG_FIFO_EN	[4] ZG_FIFO_EN	[3] ACCEL_FIFO_EN	[2] SLV2_FIFO_EN	[1] SLV1_FIFO_EN	[0] SLV0_FIFO_EN|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x24] I2C_MST_CTRL|	R/W |	[7] MULT_MST_EN	[6] WAIT_FOR_ES	[5] SLV_3_FIFO_EN	[4] I2C_MST_P_NSR	[3:0] I2C_MST_CLK|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x25] I2C_SLV0_ADDR|	R/W |	[7] I2C_SLV0_RW	[6:0] I2C_SLV0_ADDR|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x26] I2C_SLV0_REG|	R/W |	[7:0] I2C_SLV0_REG|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x27] I2C_SLV0_CTRL|	R/W |	[7] I2C_SLV0_EN	[6] I2C_SLV0_BYTE_SW	[5] I2C_SLV0_REG_DIS	[4] I2C_SLV0_GRP	[3:0] I2C_SLV0_LEN|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x28] I2C_SLV1_ADDR|	R/W |	[7] I2C_SLV1_RW	[6:0] I2C_SLV1_ADDR|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x29] I2C_SLV1_REG|	R/W |	[7:0] I2C_SLV1_REG|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x2A] I2C_SLV1_CTRL|	R/W |	[7] I2C_SLV1_EN	[6] I2C_SLV1_BYTE_SW	[5] I2C_SLV1_REG_DIS	[4] I2C_SLV1_GRP	[3:0] I2C_SLV1_LEN|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x2B] I2C_SLV2_ADDR|	R/W |	[7] I2C_SLV2_RW	[6:0] I2C_SLV2_ADDR|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x2C] I2C_SLV2_REG|	R/W |	[7:0] I2C_SLV2_REG|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x2D] I2C_SLV2_CTRL|	R/W |	[7] I2C_SLV2_EN	[6] I2C_SLV2_BYTE_SW	[5] I2C_SLV2_REG_DIS	[4] I2C_SLV2_GRP	[3:0] I2C_SLV2_LEN|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x2E] I2C_SLV3_ADDR|	R/W |	[7] I2C_SLV3_RW	[6:0] I2C_SLV3_ADDR|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x2F] I2C_SLV3_REG|	R/W |	[7:0] I2C_SLV3_REG|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x30] I2C_SLV3_CTRL|	R/W |	[7] I2C_SLV3_EN	[6] I2C_SLV3_BYTE_SW	[5] I2C_SLV3_REG_DIS	[4] I2C_SLV3_GRP	[3:0] I2C_SLV3_LEN|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x31] I2C_SLV4_ADDR|	R/W |	[7] I2C_SLV4_RW	[6:0] I2C_SLV4_ADDR|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x32] I2C_SLV4_REG|	R/W |	[7:0] I2C_SLV4_REG|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x33] I2C_SLV4_DO|	R/W |	[7:0] I2C_SLV4_DO|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x34] I2C_SLV4_CTRL|	R/W |	[7] I2C_SLV4_EN	[6] I2C_SLV4_INT_EN	[5] I2C_SLV4_REG_DIS	[4:0] I2C_MST_DLY|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x35] I2C_SLV4_DI|	R/W |	[7:0] I2C_SLV4_DI|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x36] I2C_MST_STATUS|	RO|	[7] PASS_THROUGH	[6] I2C_SLV4_DONE	[5] I2C_LOST_ARB	[4] I2C_SLV4_NACK	[3] I2C_SLV3_NACK	[2] I2C_SLV2_NACK	[1] I2C_SLV1_NACK	[0] I2C_SLV0_NACK|
| <ul><li> -[x] </li></ul>|<ul><li> -[ ] </li></ul>|[0x37] INT_PIN_CFG|	R/W |	[7] INT_LEVEL	[6] INT_OPEN	[5] LATCH_INT_EN	[4] INT_RD_CLEAR	[3] FSYNC_INT_LEVEL	[2] FSYNC_INT_EN	[1] I2C_BYPASS_EN	[0] CLKOUT_EN|
| <ul><li> -[x] </li></ul>|<ul><li> -[ ] </li></ul>|[0x38] INT_ENABLE|	R/W |	[7] FF_EN	[6] MOT_EN	[5] ZMOT_EN	[4] FIFO_OFLOW_EN	[3] I2C_MST_INT_EN	[2] PLL_RDY_INT_EN	[1] DMP_INT_EN	[0] RAW_RDY_EN|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x39] DMP_INT_STATUS|	RO|	 	[5] DMP_INT_5	[4] DMP_INT_4	[3] DMP_INT_3	[2] DMP_INT_2	[1] DMP_INT_1	[0] DMP_INT_0|
| <ul><li> -[x] </li></ul>|<ul><li> -[ ] </li></ul>|[0x3A] INT_STATUS|	RO|	[7] FF_INT	[6] MOT_INT	[5] ZMOT_INT	[4] FIFO_OFLOW_INT	[3] I2C_MST_INT	[2] PLL_RDY_INT	[1] DMP_INT	[0] RAW_RDY_INT|
| <ul><li> -[x] </li></ul>|<ul><li> -[x] </li></ul>|[0x3B] ACCEL_XOUT_H|	RO|	[15:0] ACCEL_XOUT|
| <ul><li> -[x] </li></ul>|<ul><li> -[x] </li></ul>|[0x3C] ACCEL_XOUT_L|	RO|
| <ul><li> -[x] </li></ul>|<ul><li> -[x] </li></ul>|[0x3D] ACCEL_YOUT_H|	RO|	[15:0] ACCEL_YOUT|
| <ul><li> -[x] </li></ul>|<ul><li> -[x] </li></ul>|[0x3E] ACCEL_YOUT_L|	RO|
| <ul><li> -[x] </li></ul>|<ul><li> -[x] </li></ul>|[0x3F] ACCEL_ZOUT_H|	RO|	[15:0] ACCEL_ZOUT|
| <ul><li> -[x] </li></ul>|<ul><li> -[x] </li></ul>|[0x40] ACCEL_ZOUT_L|	RO||
| <ul><li> -[x] </li></ul>|<ul><li> -[x] </li></ul>|[0x41] TEMP_OUT_H|	RO|	[15:0] TEMP_OUT|
| <ul><li> -[x] </li></ul>|<ul><li> -[x] </li></ul>|[0x42] TEMP_OUT_L|	RO||
| <ul><li> -[x] </li></ul>|<ul><li> -[x] </li></ul>|[0x43] GYRO_XOUT_H|	RO | [15:0] GYRO_XOUT|
| <ul><li> -[x] </li></ul>|<ul><li> -[x] </li></ul>|[0x44] GYRO_XOUT_L|	RO||
| <ul><li> -[x] </li></ul>|<ul><li> -[x] </li></ul>|[0x45] GYRO_YOUT_H|	RO|	[15:0] GYRO_YOUT|
| <ul><li> -[x] </li></ul>|<ul><li> -[x] </li></ul>|[0x46] GYRO_YOUT_L|	RO||
| <ul><li> -[x] </li></ul>|<ul><li> -[x] </li></ul>|[0x47] GYRO_ZOUT_H|	RO|	[15:0] GYRO_ZOUT|
| <ul><li> -[x] </li></ul>|<ul><li> -[x] </li></ul>|[0x48] GYRO_ZOUT_L|	RO||
| <ul><li> -[ ] </ul></li>|<ul><li> -[ ] </li></ul>|[0x49] EXT_SENS_DATA_00|	RO	|[7:0] EXT_SENS_DATA_00|
| <ul><li> -[ ] </ul></li>|<ul><li> -[ ] </li></ul>|[0x4A] EXT_SENS_DATA_01|	RO	|[7:0] EXT_SENS_DATA_01|
| <ul><li> -[ ] </ul></li>|<ul><li> -[ ] </li></ul>|[0x4B] EXT_SENS_DATA_02|	RO	|[7:0] EXT_SENS_DATA_02|
| <ul><li> -[ ] </ul></li>|<ul><li> -[ ] </li></ul>|[0x4C] EXT_SENS_DATA_03|	RO	|[7:0] EXT_SENS_DATA_03|
| <ul><li> -[ ] </ul></li>|<ul><li> -[ ] </li></ul>|[0x4D] EXT_SENS_DATA_04|	RO	|[7:0] EXT_SENS_DATA_04|
| <ul><li> -[ ] </ul></li>|<ul><li> -[ ] </li></ul>|[0x4E] EXT_SENS_DATA_05|	RO	|[7:0] EXT_SENS_DATA_05|
| <ul><li> -[ ] </ul></li>|<ul><li> -[ ] </li></ul>|[0x4F] EXT_SENS_DATA_06|	RO	|[7:0] EXT_SENS_DATA_06|
| <ul><li> -[ ] </ul></li>|<ul><li> -[ ] </li></ul>|[0x50] EXT_SENS_DATA_07|	RO	|[7:0] EXT_SENS_DATA_07|
| <ul><li> -[ ] </ul></li>|<ul><li> -[ ] </li></ul>|[0x51] EXT_SENS_DATA_08|	RO	|[7:0] EXT_SENS_DATA_08|
| <ul><li> -[ ] </ul></li>|<ul><li> -[ ] </li></ul>|[0x52] EXT_SENS_DATA_09|	RO	|[7:0] EXT_SENS_DATA_09|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x53] EXT_SENS_DATA_10|	RO	|[7:0] EXT_SENS_DATA_10|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x54] EXT_SENS_DATA_11|	RO	|[7:0] EXT_SENS_DATA_11|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x55] EXT_SENS_DATA_12|	RO	|[7:0] EXT_SENS_DATA_12|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x56] EXT_SENS_DATA_13|	RO	|[7:0] EXT_SENS_DATA_13|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x57] EXT_SENS_DATA_14|	RO	|[7:0] EXT_SENS_DATA_14|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x58] EXT_SENS_DATA_15|	RO	|[7:0] EXT_SENS_DATA_15|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x59] EXT_SENS_DATA_16|	RO	|[7:0] EXT_SENS_DATA_16|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x5A] EXT_SENS_DATA_17|	RO	|[7:0] EXT_SENS_DATA_17|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x5B] EXT_SENS_DATA_18|	RO	|[7:0] EXT_SENS_DATA_18|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x5C] EXT_SENS_DATA_19|	RO	|[7:0] EXT_SENS_DATA_19|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x5D] EXT_SENS_DATA_20|	RO	|[7:0] EXT_SENS_DATA_20|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x5E] EXT_SENS_DATA_21|	RO	|[7:0] EXT_SENS_DATA_21|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x5F] EXT_SENS_DATA_22|	RO	|[7:0] EXT_SENS_DATA_22|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x60] EXT_SENS_DATA_23|	RO	|[7:0] EXT_SENS_DATA_23|
| <ul><li> -[x] </li></ul>|<ul><li> -[ ] </li></ul>|[0x61] MOT_DETECT_STATUS|	RO	|[7] MOT_XNEG	[6] MOT_XPOS	[5] MOT_YNEG	[4] MOT_YPOS	[3] MOT_ZNEG	[2] MOT_ZPOS	 	[0] MOT_ZRMOT|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x63] I2C_SLV0_DO|	R/W |	[7:0] I2C_SLV0_DO|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x64] I2C_SLV1_DO|	R/W |	[7:0] I2C_SLV1_DO|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x65] I2C_SLV2_DO|	R/W |	[7:0] I2C_SLV2_DO|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x66] I2C_SLV3_DO|	R/W |	[7:0] I2C_SLV3_DO|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x67] I2C_MST_DELAY_CTRL|	R/W |	[7] DELAY_ES_SHADOW	 	[4] I2C_SLV4_DLY_EN	[3] I2C_SLV3_DLY_EN	[2] I2C_SLV2_DLY_EN	[1] I2C_SLV1_DLY_EN	[0] I2C_SLV0_DLY_EN|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x68] SIGNAL_PATH_RESET|	R/W |	 	[2] GYRO_RESET	[1] ACCEL_RESET	[0] TEMP_RESET|
| <ul><li> -[x] </li></ul>|<ul><li> -[ ] </li></ul>|[0x69] MOT_DETECT_CTRL|	R/W |	 	[5:4] ACCEL_ON_DELAY	[3:2] FF_COUNT	[1:0] MOT_COUNT|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x6A] USER_CTRL|	R/W |	[7] DMP_EN	[6] FIFO_EN	[5] I2C_MST_EN	[4] I2C_IF_DIS	[3] DMP_RESET	[2] FIFO_RESET	[1] I2C_MST_RESET	[0] SIG_COND_RESET|
| <ul><li> -[x] </li></ul>|<ul><li> -[x] </li></ul>|[0x6B] PWR_MGMT_1|	R/W |	[7] DEVICE_RESET	[6] SLEEP	[5] CYCLE	 	[3] TEMP_DIS	[2:0] CLK_SEL|
| <ul><li> -[x] </li></ul>|<ul><li> -[ ] </li></ul>|[0x6C] PWR_MGMT_2|	R/W |	[7] LP_WAKE_CTRL	 	[5] STBY_ZG	[4] STBY_YA	[3] STBY_ZA	[2] STBY_XG	[1] STBY_YG	[0] STBY_ZG|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x6D] BANK_SEL|	R/W |	 	[6] PRFTCH_EN	[5] CFG_USER_BANK	[4:0] MEM_SEL|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x6E] MEM_START_ADDR|	R/W |	[7:0] START_ADDR|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x6F] MEM_R_W|	R/W |	[7:0] MEM_R_W|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x70] DMP_CFG_1|	R/W |	 |
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x71] DMP_CFG_2|	R/W |	 |
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x72] FIFO_COUNTH|	R/W |	[15:0] FIFO_COUNT|
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x73] FIFO_COUNTL|	R/W ||
| <ul><li> -[ ] </li></ul>|<ul><li> -[ ] </li></ul>|[0x74] FIFO_R_W|	R/W |	[7:0] FIFO_R_W|
| <ul><li> -[x] </li></ul>|<ul><li> -[ ] </li></ul>|[0x75] WHO_AM_I|	RO	| 	[6:1] WHO_AM_I	| 
