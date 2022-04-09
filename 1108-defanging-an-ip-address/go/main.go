package main

import "strings"

func defangIPaddr(address string) string {
	return strings.Replace(address, ".", "[.]", -1)
}

func defangIPaddr(address string) string {
	sb := strings.Builder{}
	for i := 0; i < len(address); i++ {
		c := address[i]
		if c != '.' {
			sb.WriteByte(c)
		} else {
			sb.Write([]byte("[.]"))
		}
	}
	return sb.String()
}

//用[]byte 实现 : byte 是 uint8 的别名，占 8 位，所以处理 ASCII 没问题
func defangIPaddr(address string) string {
	origin := []byte(address)
	n := len(origin)
	newN := n + 2*3
	newString := make([]byte, newN)
	k := 0
	for i := 0; i < n; i++ {
		if origin[i] != '.' {
			newString[k] = origin[i]
			k++
		} else {
			newString[k] = '[' //go 中 newString[k++] 编译不通过
			k++
			newString[k] = '.'
			k++
			newString[k] = ']'
			k++
		}
	}
	return string(newString)
}

//用[]rune：rune 是 int32 的别名，占 32 位，4 个字节，如果处理中文字符更适合用 rune
func defangIPaddr(address string) string {
	origin := []rune(address)
	n := len(origin)
	newN := n + 2*3
	newString := make([]rune, newN)
	k := 0
	for i := 0; i < n; i++ {
		if origin[i] != '.' {
			newString[k] = origin[i]
			k++
		} else {
			newString[k] = '['
			k++
			newString[k] = '.'
			k++
			newString[k] = ']'
			k++
		}
	}
	return string(newString)
}
