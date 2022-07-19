Imports System.Security.Cryptography
Imports System.IO
Imports MySql.Data.MySqlClient
Module functions
    Public frndusrname As String
    Public connection As New MySqlConnection
    Public tempcon As New MySqlConnection
    Public msgsubject As String = String.Empty
    Public imageString As String
    Dim conString As String
    Sub tempconopen()
        tempcon.ConnectionString = conString
        Try
            tempcon.Open()
        Catch ex As Exception
            MsgBox(ex.Message & vbNewLine & "Проверьте интернет-соединение." & vbNewLine & "Приложение будет закрыто.", MsgBoxStyle.Critical, "БД недоступна")
            Application.Exit()
            End
        End Try
    End Sub

    Sub set_con_string(host As String, port As String, user As String, password As String, dbname As String)

        conString = "server=" + host + "; port=" + port + "; user id=" + user + "; password=" + password + "; database=" + dbname

    End Sub
    Sub Create_DB(host As String, port As String, user As String, password As String, dbName As String)

        Dim conn As MySqlConnection
        Dim strConn As String
        Dim cmd As MySqlCommand

        strConn = "Server =" + host + "; port=" + port + "; userid = " + user + "; password =" + password + ";"
        strConn &= "Database = mysql; pooling=false;"

        conn = New MySqlConnection(strConn)

        cmd = New MySqlCommand("Создать БД, если не существует " + dbName, conn)

        Try
            conn.Open()

        Catch ex As Exception
            MsgBox(ex.Message & vbNewLine & "Нет соединения с хостом (" & host & ") по указанному порту (" & port & ") с данным пользователем (" & user & ") и паролем." & vbNewLine & "Приложение будет закрыто.", MsgBoxStyle.Critical, "Хост недоступен")
            Application.Exit()
            End
        End Try

        Try
            cmd.ExecuteNonQuery()

        Catch ex As Exception
            MsgBox(ex.Message, MsgBoxStyle.Critical, "Ошибка БД")
            Application.Exit()
            End
        Finally
            conn.Close()
        End Try
    End Sub


    Sub InitilizeDatabase()
        Dim insertcommand As New MySqlCommand
        insertcommand = New MySqlCommand("CREATE TABLE `tb_chat` (`id` int(11) NOT NULL,`chat` longtext NOT NULL) ENGINE=InnoDB DEFAULT CHARSET=latin1;", connection)
        Try
            insertcommand.ExecuteNonQuery()
        Catch ex As Exception
            If (Not ex.Message = "Table 'tb_chat' already exists") Then
                MsgBox(ex.Message, MsgBoxStyle.Critical, "Ошибка чтения БД")
            End If

        End Try

        insertcommand = New MySqlCommand("CREATE TABLE `tb_pm` (`id` int(11) NOT NULL,`username` varchar(200) NOT NULL,`fromuser` varchar(200) NOT NULL,`subject` varchar(300) NOT NULL,`message` text NOT NULL,`isread` int(11) NOT NULL) ENGINE=InnoDB DEFAULT CHARSET=latin1;", connection)
        Try
            insertcommand.ExecuteNonQuery()
        Catch ex As Exception
            If (Not ex.Message = "Table 'tb_pm' already exists") Then
                MsgBox(ex.Message, MsgBoxStyle.Critical, "Ошибка чтения БД")
            End If
        End Try

        insertcommand = New MySqlCommand("CREATE TABLE `tb_users` (`id` int(11) NOT NULL,`username` varchar(200) NOT NULL,`password` varchar(200) NOT NULL,`email` varchar(200) NOT NULL,`name` varchar(200) NOT NULL,`dp` longtext NOT NULL,`mobile` varchar(50) NOT NULL,`chatcolor` varchar(100) NOT NULL,`chatfont` varchar(100) NOT NULL,`chatfontsize` int(11) NOT NULL,`chatfontstyle` varchar(40) NOT NULL) ENGINE=InnoDB DEFAULT CHARSET=latin1;", connection)
        Try
            insertcommand.ExecuteNonQuery()
            insertcommand = New MySqlCommand("INSERT INTO `tb_chat` (`id`, `chat`) VALUES (1, 'Добро пожаловать в Таверну!');", connection)
            insertcommand.ExecuteNonQuery()

        Catch ex As Exception
            If (Not ex.Message = "Table 'tb_users' already exists") Then
                MsgBox(ex.Message, MsgBoxStyle.Critical, "Ошибка чтения БД")
            End If
        End Try

        insertcommand = New MySqlCommand("ALTER TABLE `tb_chat` ADD PRIMARY KEY (`id`);", connection)
        Try
            insertcommand.ExecuteNonQuery()
        Catch ex As Exception
            If (Not ex.Message = "Multiple primary key defined") Then
                MsgBox(ex.Message, MsgBoxStyle.Critical, "Ошибка чтения БД")
            End If
        End Try

        insertcommand = New MySqlCommand("ALTER TABLE `tb_pm` ADD PRIMARY KEY (`id`);", connection)
        Try
            insertcommand.ExecuteNonQuery()
        Catch ex As Exception
            If (Not ex.Message = "Multiple primary key defined") Then
                MsgBox(ex.Message, MsgBoxStyle.Critical, "Ошибка чтения БД")
            End If
        End Try

        insertcommand = New MySqlCommand("ALTER TABLE `tb_users` ADD PRIMARY KEY (`id`);", connection)
        Try
            insertcommand.ExecuteNonQuery()
        Catch ex As Exception
            If (Not ex.Message = "Multiple primary key defined") Then
                MsgBox(ex.Message, MsgBoxStyle.Critical, "Ошибка чтения БД")
            End If
        End Try

        insertcommand = New MySqlCommand("ALTER TABLE `tb_pm` MODIFY `id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=6;", connection)
        Try
            insertcommand.ExecuteNonQuery()
        Catch ex As Exception
            If (Not ex.Message = "Multiple primary key defined") Then
                MsgBox(ex.Message, MsgBoxStyle.Critical, "Ошибка чтения БД")
            End If
        End Try

        insertcommand = New MySqlCommand("ALTER TABLE `tb_users` MODIFY `id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=19;", connection)
        Try
            insertcommand.ExecuteNonQuery()
        Catch ex As Exception
            If (Not ex.Message = "Multiple primary key defined") Then
                MsgBox(ex.Message, MsgBoxStyle.Critical, "Ошибка чтения БД")
            End If
        End Try

    End Sub


    Sub openconnection()
        connection.ConnectionString = conString
        Try
            connection.Open()
        Catch ex As Exception
            MsgBox("Ошибка соеднинения с сервером" & vbNewLine & "Проверьте интернет-соединение и соединение с БД." & vbNewLine & "Приложение будет закрыто.", MsgBoxStyle.Critical, "БД недоступна")
            Application.Exit()
            End
        End Try

    End Sub

    Function md5encrypter(ByVal strPass As String) As String
        'Credit goes to dreamincode.net

        Dim hasher As New MD5CryptoServiceProvider
        Dim passbytes As Byte() = New Byte(strPass.Length + 3) {}
        Dim hashBytes As Byte()

        For i As Integer = 0 To strPass.Length - 1
            passbytes(i) = CByte(Asc(strPass(i)))
        Next
        passbytes(strPass.Length) = CByte(90)
        passbytes(strPass.Length + 1) = CByte(85)
        passbytes(strPass.Length + 2) = CByte(66)
        passbytes(strPass.Length + 3) = CByte(73)

        hashBytes = hasher.ComputeHash(passbytes)

        Dim newhashBytes As Byte() = New Byte(hashBytes.Length + 3) {}

        For j As Integer = 0 To hashBytes.Length - 1
            newhashBytes(j) = hashBytes(j)
        Next
        newhashBytes(hashBytes.Length) = CByte(90)
        newhashBytes(hashBytes.Length + 1) = CByte(85)
        newhashBytes(hashBytes.Length + 2) = CByte(66)
        newhashBytes(hashBytes.Length + 3) = CByte(73)

        strPass = Convert.ToBase64String(newhashBytes)
        Return strPass
    End Function

    Function imgtobytes(ByVal imgPath As String) As String
        Try
            Dim dp_image As Image = Image.FromFile(imgPath)
            Dim mems As MemoryStream = New MemoryStream()
            dp_image.Save(mems, System.Drawing.Imaging.ImageFormat.Png)
            Dim imgbytes As Byte() = mems.ToArray()

            Dim strImg As String = Convert.ToBase64String(imgbytes)

            Return strImg


        Catch ex As Exception
            MsgBox(ex.Message & vbNewLine & "Не удалось преобразовать изображение в байты.", MsgBoxStyle.Critical, "Произошла ошибка")
        End Try

        Return String.Empty
    End Function
End Module

