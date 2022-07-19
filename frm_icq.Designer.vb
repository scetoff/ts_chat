<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()> _
Partial Class frm_icq
    Inherits System.Windows.Forms.Form

    'Форма переопределяет dispose для очистки списка компонентов.
    <System.Diagnostics.DebuggerNonUserCode()> _
    Protected Overrides Sub Dispose(ByVal disposing As Boolean)
        Try
            If disposing AndAlso components IsNot Nothing Then
                components.Dispose()
            End If
        Finally
            MyBase.Dispose(disposing)
        End Try
    End Sub

    'Является обязательной для конструктора форм Windows Forms
    Private components As System.ComponentModel.IContainer

    'Примечание: следующая процедура является обязательной для конструктора форм Windows Forms
    'Для ее изменения используйте конструктор форм Windows Form.  
    'Не изменяйте ее в редакторе исходного кода.
    <System.Diagnostics.DebuggerStepThrough()> _
    Private Sub InitializeComponent()
        Me.components = New System.ComponentModel.Container()
        Dim resources As System.ComponentModel.ComponentResourceManager = New System.ComponentModel.ComponentResourceManager(GetType(frm_icq))
        Me.trei = New System.Windows.Forms.PictureBox()
        Me.EndIt = New System.Windows.Forms.PictureBox()
        Me.ChatICO = New System.Windows.Forms.PictureBox()
        Me.Stringer = New System.Windows.Forms.PictureBox()
        Me.PassChar = New System.Windows.Forms.CheckBox()
        Me.DisConnect = New System.Windows.Forms.Button()
        Me.txt_UIN = New System.Windows.Forms.TextBox()
        Me.Coonect = New System.Windows.Forms.Button()
        Me.txt_Pass = New System.Windows.Forms.TextBox()
        Me.lbl_uin = New System.Windows.Forms.Label()
        Me.lbl_pass = New System.Windows.Forms.Label()
        Me.GB_Auth = New System.Windows.Forms.GroupBox()
        Me.GB_Message = New System.Windows.Forms.GroupBox()
        Me.cbox_spisok = New System.Windows.Forms.ComboBox()
        Me.TextSend = New System.Windows.Forms.TextBox()
        Me.Send = New System.Windows.Forms.Button()
        Me.UINSend = New System.Windows.Forms.TextBox()
        Me.lbl_who = New System.Windows.Forms.Label()
        Me.lbl_status = New System.Windows.Forms.Label()
        Me.GB_LOG = New System.Windows.Forms.GroupBox()
        Me.lbox_log = New System.Windows.Forms.ListBox()
        Me.Timer1 = New System.Windows.Forms.Timer(Me.components)
        Me.Hint_User = New System.Windows.Forms.ToolTip(Me.components)
        CType(Me.trei, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).BeginInit()
        Me.GB_Auth.SuspendLayout()
        Me.GB_Message.SuspendLayout()
        Me.GB_LOG.SuspendLayout()
        Me.SuspendLayout()
        '
        'trei
        '
        Me.trei.BackColor = System.Drawing.Color.Navy
        Me.trei.Image = Global.TSchat.My.Resources.Resources.remove
        Me.trei.Location = New System.Drawing.Point(343, 0)
        Me.trei.Name = "trei"
        Me.trei.Size = New System.Drawing.Size(35, 35)
        Me.trei.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.trei.TabIndex = 16
        Me.trei.TabStop = False
        '
        'EndIt
        '
        Me.EndIt.BackColor = System.Drawing.Color.Navy
        Me.EndIt.Image = Global.TSchat.My.Resources.Resources.clear_search
        Me.EndIt.Location = New System.Drawing.Point(384, 0)
        Me.EndIt.Name = "EndIt"
        Me.EndIt.Size = New System.Drawing.Size(35, 35)
        Me.EndIt.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.EndIt.TabIndex = 15
        Me.EndIt.TabStop = False
        '
        'ChatICO
        '
        Me.ChatICO.BackColor = System.Drawing.Color.Navy
        Me.ChatICO.Image = Global.TSchat.My.Resources.Resources.ICOCOS1
        Me.ChatICO.Location = New System.Drawing.Point(-1, 0)
        Me.ChatICO.Name = "ChatICO"
        Me.ChatICO.Size = New System.Drawing.Size(35, 35)
        Me.ChatICO.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.ChatICO.TabIndex = 14
        Me.ChatICO.TabStop = False
        '
        'Stringer
        '
        Me.Stringer.BackColor = System.Drawing.Color.Navy
        Me.Stringer.Location = New System.Drawing.Point(-1, 0)
        Me.Stringer.Name = "Stringer"
        Me.Stringer.Size = New System.Drawing.Size(432, 35)
        Me.Stringer.TabIndex = 13
        Me.Stringer.TabStop = False
        '
        'PassChar
        '
        Me.PassChar.AutoSize = True
        Me.PassChar.BackColor = System.Drawing.Color.Black
        Me.PassChar.FlatStyle = System.Windows.Forms.FlatStyle.Flat
        Me.PassChar.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.PassChar.ForeColor = System.Drawing.Color.White
        Me.PassChar.Location = New System.Drawing.Point(98, 106)
        Me.PassChar.Name = "PassChar"
        Me.PassChar.Size = New System.Drawing.Size(156, 27)
        Me.PassChar.TabIndex = 23
        Me.PassChar.Text = "Показать пароль"
        Me.PassChar.UseVisualStyleBackColor = False
        '
        'DisConnect
        '
        Me.DisConnect.BackColor = System.Drawing.Color.Black
        Me.DisConnect.Cursor = System.Windows.Forms.Cursors.Hand
        Me.DisConnect.FlatStyle = System.Windows.Forms.FlatStyle.Flat
        Me.DisConnect.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.DisConnect.ForeColor = System.Drawing.Color.White
        Me.DisConnect.Location = New System.Drawing.Point(260, 68)
        Me.DisConnect.Name = "DisConnect"
        Me.DisConnect.Size = New System.Drawing.Size(144, 30)
        Me.DisConnect.TabIndex = 22
        Me.DisConnect.Text = "Отключиться"
        Me.DisConnect.UseVisualStyleBackColor = False
        '
        'txt_UIN
        '
        Me.txt_UIN.BackColor = System.Drawing.Color.White
        Me.txt_UIN.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle
        Me.txt_UIN.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.txt_UIN.ForeColor = System.Drawing.Color.Black
        Me.txt_UIN.Location = New System.Drawing.Point(98, 28)
        Me.txt_UIN.MaxLength = 10
        Me.txt_UIN.Name = "txt_UIN"
        Me.txt_UIN.Size = New System.Drawing.Size(156, 30)
        Me.txt_UIN.TabIndex = 19
        '
        'Coonect
        '
        Me.Coonect.BackColor = System.Drawing.Color.Black
        Me.Coonect.Cursor = System.Windows.Forms.Cursors.Hand
        Me.Coonect.FlatStyle = System.Windows.Forms.FlatStyle.Flat
        Me.Coonect.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.Coonect.ForeColor = System.Drawing.Color.White
        Me.Coonect.Location = New System.Drawing.Point(260, 28)
        Me.Coonect.Name = "Coonect"
        Me.Coonect.Size = New System.Drawing.Size(144, 30)
        Me.Coonect.TabIndex = 17
        Me.Coonect.Text = "Подключиться"
        Me.Coonect.UseVisualStyleBackColor = False
        '
        'txt_Pass
        '
        Me.txt_Pass.BackColor = System.Drawing.Color.White
        Me.txt_Pass.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle
        Me.txt_Pass.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.txt_Pass.ForeColor = System.Drawing.Color.Black
        Me.txt_Pass.Location = New System.Drawing.Point(98, 68)
        Me.txt_Pass.MaxLength = 20
        Me.txt_Pass.Name = "txt_Pass"
        Me.txt_Pass.PasswordChar = Global.Microsoft.VisualBasic.ChrW(42)
        Me.txt_Pass.Size = New System.Drawing.Size(156, 30)
        Me.txt_Pass.TabIndex = 21
        '
        'lbl_uin
        '
        Me.lbl_uin.AutoSize = True
        Me.lbl_uin.BackColor = System.Drawing.Color.Black
        Me.lbl_uin.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_uin.ForeColor = System.Drawing.Color.White
        Me.lbl_uin.Location = New System.Drawing.Point(17, 33)
        Me.lbl_uin.Name = "lbl_uin"
        Me.lbl_uin.Size = New System.Drawing.Size(54, 23)
        Me.lbl_uin.TabIndex = 18
        Me.lbl_uin.Text = "UIN :"
        '
        'lbl_pass
        '
        Me.lbl_pass.AutoSize = True
        Me.lbl_pass.BackColor = System.Drawing.Color.Black
        Me.lbl_pass.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_pass.ForeColor = System.Drawing.Color.White
        Me.lbl_pass.Location = New System.Drawing.Point(6, 70)
        Me.lbl_pass.Name = "lbl_pass"
        Me.lbl_pass.Size = New System.Drawing.Size(79, 23)
        Me.lbl_pass.TabIndex = 20
        Me.lbl_pass.Text = "Пароль :"
        '
        'GB_Auth
        '
        Me.GB_Auth.Controls.Add(Me.lbl_uin)
        Me.GB_Auth.Controls.Add(Me.DisConnect)
        Me.GB_Auth.Controls.Add(Me.PassChar)
        Me.GB_Auth.Controls.Add(Me.txt_Pass)
        Me.GB_Auth.Controls.Add(Me.txt_UIN)
        Me.GB_Auth.Controls.Add(Me.Coonect)
        Me.GB_Auth.Controls.Add(Me.lbl_pass)
        Me.GB_Auth.Font = New System.Drawing.Font("Comic Sans MS", 12.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.GB_Auth.ForeColor = System.Drawing.Color.White
        Me.GB_Auth.Location = New System.Drawing.Point(6, 59)
        Me.GB_Auth.Name = "GB_Auth"
        Me.GB_Auth.Size = New System.Drawing.Size(413, 138)
        Me.GB_Auth.TabIndex = 24
        Me.GB_Auth.TabStop = False
        Me.GB_Auth.Text = "Авторизация"
        '
        'GB_Message
        '
        Me.GB_Message.Controls.Add(Me.cbox_spisok)
        Me.GB_Message.Controls.Add(Me.TextSend)
        Me.GB_Message.Controls.Add(Me.Send)
        Me.GB_Message.Controls.Add(Me.UINSend)
        Me.GB_Message.Controls.Add(Me.lbl_who)
        Me.GB_Message.Font = New System.Drawing.Font("Comic Sans MS", 12.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.GB_Message.ForeColor = System.Drawing.Color.White
        Me.GB_Message.Location = New System.Drawing.Point(6, 203)
        Me.GB_Message.Name = "GB_Message"
        Me.GB_Message.Size = New System.Drawing.Size(413, 219)
        Me.GB_Message.TabIndex = 25
        Me.GB_Message.TabStop = False
        Me.GB_Message.Text = "Сообщения"
        '
        'cbox_spisok
        '
        Me.cbox_spisok.BackColor = System.Drawing.Color.SeaShell
        Me.cbox_spisok.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList
        Me.cbox_spisok.ForeColor = System.Drawing.Color.Black
        Me.cbox_spisok.FormattingEnabled = True
        Me.cbox_spisok.Items.AddRange(New Object() {"В сети", "Не в сети", "Злой", "Отсутствую", "Депрессия", "Недоступен", "Занят", "Не беспокоить", "На работе", "Дома", "Кушаю", "Невидимый", "Нет статуса"})
        Me.cbox_spisok.Location = New System.Drawing.Point(6, 181)
        Me.cbox_spisok.Name = "cbox_spisok"
        Me.cbox_spisok.Size = New System.Drawing.Size(162, 32)
        Me.cbox_spisok.TabIndex = 9
        '
        'TextSend
        '
        Me.TextSend.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle
        Me.TextSend.Location = New System.Drawing.Point(6, 62)
        Me.TextSend.Multiline = True
        Me.TextSend.Name = "TextSend"
        Me.TextSend.Size = New System.Drawing.Size(400, 113)
        Me.TextSend.TabIndex = 5
        '
        'Send
        '
        Me.Send.BackColor = System.Drawing.Color.Black
        Me.Send.Cursor = System.Windows.Forms.Cursors.Hand
        Me.Send.FlatStyle = System.Windows.Forms.FlatStyle.Flat
        Me.Send.ForeColor = System.Drawing.Color.White
        Me.Send.Location = New System.Drawing.Point(198, 180)
        Me.Send.Name = "Send"
        Me.Send.Size = New System.Drawing.Size(209, 32)
        Me.Send.TabIndex = 8
        Me.Send.Text = "Отправить сообщение"
        Me.Send.UseVisualStyleBackColor = False
        '
        'UINSend
        '
        Me.UINSend.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle
        Me.UINSend.ForeColor = System.Drawing.Color.Black
        Me.UINSend.Location = New System.Drawing.Point(77, 25)
        Me.UINSend.Name = "UINSend"
        Me.UINSend.Size = New System.Drawing.Size(329, 31)
        Me.UINSend.TabIndex = 3
        '
        'lbl_who
        '
        Me.lbl_who.AutoSize = True
        Me.lbl_who.BackColor = System.Drawing.Color.Black
        Me.lbl_who.ForeColor = System.Drawing.Color.White
        Me.lbl_who.Location = New System.Drawing.Point(9, 27)
        Me.lbl_who.Name = "lbl_who"
        Me.lbl_who.Size = New System.Drawing.Size(62, 24)
        Me.lbl_who.TabIndex = 0
        Me.lbl_who.Text = "Кому :"
        '
        'lbl_status
        '
        Me.lbl_status.AutoSize = True
        Me.lbl_status.BackColor = System.Drawing.Color.Black
        Me.lbl_status.Font = New System.Drawing.Font("Comic Sans MS", 9.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_status.ForeColor = System.Drawing.Color.White
        Me.lbl_status.Location = New System.Drawing.Point(237, 38)
        Me.lbl_status.Name = "lbl_status"
        Me.lbl_status.Size = New System.Drawing.Size(61, 18)
        Me.lbl_status.TabIndex = 26
        Me.lbl_status.Text = "Статус : "
        '
        'GB_LOG
        '
        Me.GB_LOG.BackColor = System.Drawing.Color.Black
        Me.GB_LOG.Controls.Add(Me.lbox_log)
        Me.GB_LOG.Font = New System.Drawing.Font("Comic Sans MS", 12.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.GB_LOG.ForeColor = System.Drawing.Color.White
        Me.GB_LOG.Location = New System.Drawing.Point(6, 428)
        Me.GB_LOG.Name = "GB_LOG"
        Me.GB_LOG.Size = New System.Drawing.Size(413, 172)
        Me.GB_LOG.TabIndex = 27
        Me.GB_LOG.TabStop = False
        Me.GB_LOG.Text = "Лог сообщений"
        '
        'lbox_log
        '
        Me.lbox_log.BackColor = System.Drawing.Color.White
        Me.lbox_log.Font = New System.Drawing.Font("Comic Sans MS", 9.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbox_log.FormattingEnabled = True
        Me.lbox_log.HorizontalScrollbar = True
        Me.lbox_log.ItemHeight = 18
        Me.lbox_log.Location = New System.Drawing.Point(6, 30)
        Me.lbox_log.Name = "lbox_log"
        Me.lbox_log.ScrollAlwaysVisible = True
        Me.lbox_log.Size = New System.Drawing.Size(401, 130)
        Me.lbox_log.TabIndex = 11
        '
        'Timer1
        '
        Me.Timer1.Interval = 1
        '
        'frm_icq
        '
        Me.AutoScaleDimensions = New System.Drawing.SizeF(6.0!, 13.0!)
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.BackColor = System.Drawing.Color.Black
        Me.ClientSize = New System.Drawing.Size(424, 604)
        Me.Controls.Add(Me.GB_LOG)
        Me.Controls.Add(Me.lbl_status)
        Me.Controls.Add(Me.GB_Message)
        Me.Controls.Add(Me.GB_Auth)
        Me.Controls.Add(Me.trei)
        Me.Controls.Add(Me.EndIt)
        Me.Controls.Add(Me.ChatICO)
        Me.Controls.Add(Me.Stringer)
        Me.ForeColor = System.Drawing.Color.White
        Me.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None
        Me.Icon = CType(resources.GetObject("$this.Icon"), System.Drawing.Icon)
        Me.MaximizeBox = False
        Me.Name = "frm_icq"
        Me.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen
        Me.Text = "Аська"
        CType(Me.trei, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).EndInit()
        Me.GB_Auth.ResumeLayout(False)
        Me.GB_Auth.PerformLayout()
        Me.GB_Message.ResumeLayout(False)
        Me.GB_Message.PerformLayout()
        Me.GB_LOG.ResumeLayout(False)
        Me.ResumeLayout(False)
        Me.PerformLayout()

    End Sub

    Friend WithEvents trei As PictureBox
    Friend WithEvents EndIt As PictureBox
    Friend WithEvents ChatICO As PictureBox
    Friend WithEvents Stringer As PictureBox
    Friend WithEvents PassChar As CheckBox
    Friend WithEvents DisConnect As Button
    Friend WithEvents txt_UIN As TextBox
    Friend WithEvents Coonect As Button
    Friend WithEvents txt_Pass As TextBox
    Friend WithEvents lbl_uin As Label
    Friend WithEvents lbl_pass As Label
    Friend WithEvents GB_Auth As GroupBox
    Friend WithEvents GB_Message As GroupBox
    Friend WithEvents cbox_spisok As ComboBox
    Friend WithEvents TextSend As TextBox
    Friend WithEvents Send As Button
    Friend WithEvents UINSend As TextBox
    Friend WithEvents lbl_who As Label
    Friend WithEvents lbl_status As Label
    Friend WithEvents GB_LOG As GroupBox
    Friend WithEvents lbox_log As ListBox
    Friend WithEvents Timer1 As Timer
    Friend WithEvents Hint_User As ToolTip
End Class
