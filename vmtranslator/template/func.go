package template

const FUNC = `
({{ .Fn.GetName }})
{{- $label := .T.GetID | printf "NEWFUNC_PROCEDURE_%d" -}}
{{- if gt .Fn.GetN 0 }}
	@{{ .T.GetID }}
	D=A
	@R13
	M=D
({{ $label }})
	@SP
	A=M
	M=0
	@SP
	M=M+1
	@R13
	MD=M-1
	@{{ $label }}
	D;JGE
{{- end }}
`

const CALL = `
{{- $ret_label := printf "%s$ret.%d" .T.GetCurFnName .T.GetID -}}
	{{ printf "\t@%s" $ret_label }}
	D=A
	@SP
	A=M
	M=D // Do not advance SP here, do it in CALL_PROCEDURE
	@{{ .Fn.GetName }}
	D=A
	@R13
	M=D
	@{{ .Fn.GetN }}
	D=A
	@R14
	M=D
	@CALL_PROCEDURE
	0;JMP
({{ $ret_label }})
`
