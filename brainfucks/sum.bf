; adds two numbers together
; both numbers must be under 5 to work correctly
; (aka their sum must be under between 0 and 9)

++++                    ; 4
[>>++++++++++<<-]       ; * 10
>>+++                   ; ascii plus in 3d column

[<+<+>>-]               ; duplicate it to the first 2 columns

++                      ; 2
[<----->-]              ; * negative 5
<-

>>++++++
[<++++++++++>-]
<+

                        ; currently: (1: (ascii plus) 2: (ascii space) 3: (ascii equal))

>,
>,
                        ; currently: (1: (ascii plus) 2: (ascii space) 3: (ascii equal) 4: (ascii *num1*) 5: (ascii *num2*))

<.                      ; num1
<<.                     ; space
<.                      ; plus
>.                      ; space
>>>.                    ; num2
<<<.                    ; space
>.                      ; eq
<.                      ; space

>>>>>>
++++++++++
[<+++++<+++++>>-]
<--
<--

[<-<->>-]
<
[<+>-]
>>
[<<<+>>>-]
<<<.
