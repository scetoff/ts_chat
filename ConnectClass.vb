'Imports System.Security.Cryptography
'Imports System.IO
'Imports MySql.Data.MySqlClient
'Module Con_Add
'    Public usrnameFriend As String
'    Public Conn As New MySqlConnection
'    Public ConTemp As New MySqlConnection
'    Public MsgSubject As String = String.Empty

'    Sub ConTempOpen()
'        ConTemp.ConnectionString = "server=" & My.Settings.Host & "; user id=" & My.Settings.User & "; password=" & My.Settings.Pass & "; database= " & My.Settings.dbName
'        Try
'            ConTemp.Open()
'        Catch ex As Exception
'            MsgBox(ex.Message & vbNewLine & "Проверьте интернет-соединение.", MsgBoxStyle.Critical, "Нет подключения к БД")
'            Application.Exit()
'        End Try
'    End Sub

'    Sub OpenConnection()
'        Conn.ConnectionString = "server=" & My.Settings.Host & "; user id=" & My.Settings.User & "; password=" & My.Settings.Pass & "; database= " & My.Settings.dbName
'        Try
'            Conn.Open()
'        Catch ex As Exception
'            MsgBox("Нет соединения с хостом." & vbNewLine & "Проверьте интернет-соединение." & vbNewLine & "Приложение будет закрыто.", MsgBoxStyle.Critical, "Нет подключения к БД")
'            Application.Exit()
'        End Try

'    End Sub

'    Function MD5Encrypter(ByVal strPass As String) As String
'        Credit goes to dreamincode.net

'        Dim Hasher As New MD5CryptoServiceProvider
'        Dim PasswordBytes As Byte() = New Byte(strPass.Length + 3) {}
'        Dim HashBytes As Byte()

'        For i As Integer = 0 To strPass.Length - 1
'            PasswordBytes(i) = CByte(Asc(strPass(i)))
'        Next
'        PasswordBytes(strPass.Length) = CByte(90)
'        PasswordBytes(strPass.Length + 1) = CByte(85)
'        PasswordBytes(strPass.Length + 2) = CByte(66)
'        PasswordBytes(strPass.Length + 3) = CByte(73)

'        HashBytes = Hasher.ComputeHash(PasswordBytes)

'        Dim NewHashBytes As Byte() = New Byte(HashBytes.Length + 3) {}

'        For j As Integer = 0 To HashBytes.Length - 1
'            NewHashBytes(j) = HashBytes(j)
'        Next
'        NewHashBytes(HashBytes.Length) = CByte(90)
'        NewHashBytes(HashBytes.Length + 1) = CByte(85)
'        NewHashBytes(HashBytes.Length + 2) = CByte(66)
'        NewHashBytes(HashBytes.Length + 3) = CByte(73)

'        strPass = Convert.ToBase64String(NewHashBytes)
'        Return strPass
'    End Function

'    Function ImageToBytes(ByVal imgPath As String) As String
'        Try
'            Dim Image_DP As Image = Image.FromFile(imgPath)
'            Dim Memories As MemoryStream = New MemoryStream()
'            Image_DP.Save(Memories, System.Drawing.Imaging.ImageFormat.Png)
'            Dim ImageBytes As Byte() = Memories.ToArray()

'            Dim StringImage As String = Convert.ToBase64String(ImageBytes)

'            Return StringImage


'        Catch ex As Exception
'            MsgBox(ex.Message & vbNewLine & "Неверный формат изображения.", MsgBoxStyle.Critical, "Ошибка!")
'        End Try

'    End Function
'End Module

