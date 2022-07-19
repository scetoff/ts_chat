<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()> _
Partial Class frm_EditProfile
    Inherits System.Windows.Forms.Form

    'Form overrides dispose to clean up the component list.
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

    'Required by the Windows Form Designer
    Private components As System.ComponentModel.IContainer

    'NOTE: The following procedure is required by the Windows Form Designer
    'It can be modified using the Windows Form Designer.  
    'Do not modify it using the code editor.
    <System.Diagnostics.DebuggerStepThrough()> _
    Private Sub InitializeComponent()
        Me.components = New System.ComponentModel.Container()
        Dim resources As System.ComponentModel.ComponentResourceManager = New System.ComponentModel.ComponentResourceManager(GetType(frm_EditProfile))
        Me.trei = New System.Windows.Forms.PictureBox()
        Me.EndIt = New System.Windows.Forms.PictureBox()
        Me.ChatICO = New System.Windows.Forms.PictureBox()
        Me.Stringer = New System.Windows.Forms.PictureBox()
        Me.pnl_red = New System.Windows.Forms.Panel()
        Me.lbl_redakt = New System.Windows.Forms.Label()
        Me.GB_redakt = New System.Windows.Forms.GroupBox()
        Me.btn_save = New System.Windows.Forms.Button()
        Me.GB_Pass = New System.Windows.Forms.GroupBox()
        Me.ChangeBoxX = New System.Windows.Forms.CheckBox()
        Me.txt_repeatpass = New System.Windows.Forms.TextBox()
        Me.lbl_Pass = New System.Windows.Forms.Label()
        Me.lbl_repeatpass = New System.Windows.Forms.Label()
        Me.txt_pass = New System.Windows.Forms.TextBox()
        Me.mask_phone = New System.Windows.Forms.MaskedTextBox()
        Me.lbl_phone = New System.Windows.Forms.Label()
        Me.txt_Nickname = New System.Windows.Forms.TextBox()
        Me.lbl_mail = New System.Windows.Forms.Label()
        Me.txt_mail = New System.Windows.Forms.TextBox()
        Me.lbl_Nickname = New System.Windows.Forms.Label()
        Me.txt_Name = New System.Windows.Forms.TextBox()
        Me.lbl_Name = New System.Windows.Forms.Label()
        Me.Hint_User = New System.Windows.Forms.ToolTip(Me.components)
        CType(Me.trei, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).BeginInit()
        Me.pnl_red.SuspendLayout()
        Me.GB_redakt.SuspendLayout()
        Me.GB_Pass.SuspendLayout()
        Me.SuspendLayout()
        '
        'trei
        '
        Me.trei.BackColor = System.Drawing.Color.Navy
        Me.trei.Image = Global.TSchat.My.Resources.Resources.remove
        Me.trei.Location = New System.Drawing.Point(535, 0)
        Me.trei.Name = "trei"
        Me.trei.Size = New System.Drawing.Size(35, 35)
        Me.trei.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.trei.TabIndex = 14
        Me.trei.TabStop = False
        '
        'EndIt
        '
        Me.EndIt.BackColor = System.Drawing.Color.Navy
        Me.EndIt.Image = Global.TSchat.My.Resources.Resources.clear_search
        Me.EndIt.Location = New System.Drawing.Point(576, 0)
        Me.EndIt.Name = "EndIt"
        Me.EndIt.Size = New System.Drawing.Size(35, 35)
        Me.EndIt.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.EndIt.TabIndex = 13
        Me.EndIt.TabStop = False
        '
        'ChatICO
        '
        Me.ChatICO.BackColor = System.Drawing.Color.Navy
        Me.ChatICO.Image = Global.TSchat.My.Resources.Resources.ICOCOS1
        Me.ChatICO.Location = New System.Drawing.Point(1, 0)
        Me.ChatICO.Name = "ChatICO"
        Me.ChatICO.Size = New System.Drawing.Size(35, 35)
        Me.ChatICO.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.ChatICO.TabIndex = 12
        Me.ChatICO.TabStop = False
        '
        'Stringer
        '
        Me.Stringer.BackColor = System.Drawing.Color.Navy
        Me.Stringer.Location = New System.Drawing.Point(0, 0)
        Me.Stringer.Name = "Stringer"
        Me.Stringer.Size = New System.Drawing.Size(625, 35)
        Me.Stringer.TabIndex = 11
        Me.Stringer.TabStop = False
        '
        'pnl_red
        '
        Me.pnl_red.BackColor = System.Drawing.Color.Black
        Me.pnl_red.Controls.Add(Me.lbl_redakt)
        Me.pnl_red.Location = New System.Drawing.Point(0, 35)
        Me.pnl_red.Name = "pnl_red"
        Me.pnl_red.Size = New System.Drawing.Size(625, 36)
        Me.pnl_red.TabIndex = 0
        '
        'lbl_redakt
        '
        Me.lbl_redakt.AutoSize = True
        Me.lbl_redakt.BackColor = System.Drawing.Color.Black
        Me.lbl_redakt.Font = New System.Drawing.Font("Comic Sans MS", 14.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_redakt.ForeColor = System.Drawing.Color.White
        Me.lbl_redakt.Location = New System.Drawing.Point(12, 4)
        Me.lbl_redakt.Name = "lbl_redakt"
        Me.lbl_redakt.Size = New System.Drawing.Size(229, 26)
        Me.lbl_redakt.TabIndex = 0
        Me.lbl_redakt.Text = "Редактировать профиль"
        '
        'GB_redakt
        '
        Me.GB_redakt.BackColor = System.Drawing.Color.Black
        Me.GB_redakt.Controls.Add(Me.btn_save)
        Me.GB_redakt.Controls.Add(Me.GB_Pass)
        Me.GB_redakt.Controls.Add(Me.mask_phone)
        Me.GB_redakt.Controls.Add(Me.lbl_phone)
        Me.GB_redakt.Controls.Add(Me.txt_Nickname)
        Me.GB_redakt.Controls.Add(Me.lbl_mail)
        Me.GB_redakt.Controls.Add(Me.txt_mail)
        Me.GB_redakt.Controls.Add(Me.lbl_Nickname)
        Me.GB_redakt.Controls.Add(Me.txt_Name)
        Me.GB_redakt.Controls.Add(Me.lbl_Name)
        Me.GB_redakt.ForeColor = System.Drawing.Color.White
        Me.GB_redakt.Location = New System.Drawing.Point(12, 78)
        Me.GB_redakt.Name = "GB_redakt"
        Me.GB_redakt.Size = New System.Drawing.Size(593, 294)
        Me.GB_redakt.TabIndex = 0
        Me.GB_redakt.TabStop = False
        '
        'btn_save
        '
        Me.btn_save.BackColor = System.Drawing.Color.Black
        Me.btn_save.Cursor = System.Windows.Forms.Cursors.Hand
        Me.btn_save.FlatStyle = System.Windows.Forms.FlatStyle.Flat
        Me.btn_save.Font = New System.Drawing.Font("Comic Sans MS", 14.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.btn_save.ForeColor = System.Drawing.Color.White
        Me.btn_save.Location = New System.Drawing.Point(317, 223)
        Me.btn_save.Name = "btn_save"
        Me.btn_save.Size = New System.Drawing.Size(224, 38)
        Me.btn_save.TabIndex = 7
        Me.btn_save.Text = "Сохранить изменения"
        Me.btn_save.TextAlign = System.Drawing.ContentAlignment.TopCenter
        Me.btn_save.UseVisualStyleBackColor = False
        '
        'GB_Pass
        '
        Me.GB_Pass.Controls.Add(Me.ChangeBoxX)
        Me.GB_Pass.Controls.Add(Me.txt_repeatpass)
        Me.GB_Pass.Controls.Add(Me.lbl_Pass)
        Me.GB_Pass.Controls.Add(Me.lbl_repeatpass)
        Me.GB_Pass.Controls.Add(Me.txt_pass)
        Me.GB_Pass.Location = New System.Drawing.Point(284, 19)
        Me.GB_Pass.Name = "GB_Pass"
        Me.GB_Pass.Size = New System.Drawing.Size(291, 195)
        Me.GB_Pass.TabIndex = 0
        Me.GB_Pass.TabStop = False
        '
        'ChangeBoxX
        '
        Me.ChangeBoxX.AutoSize = True
        Me.ChangeBoxX.BackColor = System.Drawing.Color.Black
        Me.ChangeBoxX.Font = New System.Drawing.Font("Comic Sans MS", 14.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.ChangeBoxX.ForeColor = System.Drawing.Color.White
        Me.ChangeBoxX.Location = New System.Drawing.Point(167, 10)
        Me.ChangeBoxX.Name = "ChangeBoxX"
        Me.ChangeBoxX.Size = New System.Drawing.Size(118, 30)
        Me.ChangeBoxX.TabIndex = 0
        Me.ChangeBoxX.Text = "Изменить"
        Me.ChangeBoxX.UseVisualStyleBackColor = False
        '
        'txt_repeatpass
        '
        Me.txt_repeatpass.BackColor = System.Drawing.Color.White
        Me.txt_repeatpass.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.txt_repeatpass.ForeColor = System.Drawing.Color.Black
        Me.txt_repeatpass.Location = New System.Drawing.Point(11, 150)
        Me.txt_repeatpass.MaxLength = 20
        Me.txt_repeatpass.Name = "txt_repeatpass"
        Me.txt_repeatpass.PasswordChar = Global.Microsoft.VisualBasic.ChrW(42)
        Me.txt_repeatpass.ReadOnly = True
        Me.txt_repeatpass.Size = New System.Drawing.Size(237, 30)
        Me.txt_repeatpass.TabIndex = 6
        '
        'lbl_Pass
        '
        Me.lbl_Pass.AutoSize = True
        Me.lbl_Pass.BackColor = System.Drawing.Color.Black
        Me.lbl_Pass.Font = New System.Drawing.Font("Comic Sans MS", 14.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_Pass.ForeColor = System.Drawing.Color.White
        Me.lbl_Pass.Location = New System.Drawing.Point(6, 59)
        Me.lbl_Pass.Name = "lbl_Pass"
        Me.lbl_Pass.Size = New System.Drawing.Size(156, 26)
        Me.lbl_Pass.TabIndex = 0
        Me.lbl_Pass.Text = "Введите пароль"
        '
        'lbl_repeatpass
        '
        Me.lbl_repeatpass.AutoSize = True
        Me.lbl_repeatpass.BackColor = System.Drawing.Color.Black
        Me.lbl_repeatpass.Font = New System.Drawing.Font("Comic Sans MS", 14.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_repeatpass.ForeColor = System.Drawing.Color.White
        Me.lbl_repeatpass.Location = New System.Drawing.Point(6, 121)
        Me.lbl_repeatpass.Name = "lbl_repeatpass"
        Me.lbl_repeatpass.Size = New System.Drawing.Size(181, 26)
        Me.lbl_repeatpass.TabIndex = 0
        Me.lbl_repeatpass.Text = "Повторите пароль"
        '
        'txt_pass
        '
        Me.txt_pass.BackColor = System.Drawing.Color.White
        Me.txt_pass.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.txt_pass.ForeColor = System.Drawing.Color.Black
        Me.txt_pass.Location = New System.Drawing.Point(11, 88)
        Me.txt_pass.MaxLength = 20
        Me.txt_pass.Name = "txt_pass"
        Me.txt_pass.ReadOnly = True
        Me.txt_pass.Size = New System.Drawing.Size(237, 30)
        Me.txt_pass.TabIndex = 5
        '
        'mask_phone
        '
        Me.mask_phone.BackColor = System.Drawing.Color.White
        Me.mask_phone.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.mask_phone.ForeColor = System.Drawing.Color.Black
        Me.mask_phone.Location = New System.Drawing.Point(24, 231)
        Me.mask_phone.Mask = "+7(999) 000-0000"
        Me.mask_phone.Name = "mask_phone"
        Me.mask_phone.Size = New System.Drawing.Size(237, 30)
        Me.mask_phone.TabIndex = 4
        Me.Hint_User.SetToolTip(Me.mask_phone, "Введите номер телефона")
        '
        'lbl_phone
        '
        Me.lbl_phone.AutoSize = True
        Me.lbl_phone.BackColor = System.Drawing.Color.Black
        Me.lbl_phone.Font = New System.Drawing.Font("Comic Sans MS", 14.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_phone.ForeColor = System.Drawing.Color.White
        Me.lbl_phone.Location = New System.Drawing.Point(19, 202)
        Me.lbl_phone.Name = "lbl_phone"
        Me.lbl_phone.Size = New System.Drawing.Size(94, 26)
        Me.lbl_phone.TabIndex = 0
        Me.lbl_phone.Text = "Телефон"
        '
        'txt_Nickname
        '
        Me.txt_Nickname.BackColor = System.Drawing.Color.White
        Me.txt_Nickname.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.txt_Nickname.ForeColor = System.Drawing.Color.Black
        Me.txt_Nickname.Location = New System.Drawing.Point(24, 107)
        Me.txt_Nickname.MaxLength = 15
        Me.txt_Nickname.Name = "txt_Nickname"
        Me.txt_Nickname.Size = New System.Drawing.Size(237, 30)
        Me.txt_Nickname.TabIndex = 2
        Me.Hint_User.SetToolTip(Me.txt_Nickname, "Укажите новый Никнейм")
        '
        'lbl_mail
        '
        Me.lbl_mail.AutoSize = True
        Me.lbl_mail.BackColor = System.Drawing.Color.Black
        Me.lbl_mail.Font = New System.Drawing.Font("Comic Sans MS", 14.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_mail.ForeColor = System.Drawing.Color.White
        Me.lbl_mail.Location = New System.Drawing.Point(19, 140)
        Me.lbl_mail.Name = "lbl_mail"
        Me.lbl_mail.Size = New System.Drawing.Size(69, 26)
        Me.lbl_mail.TabIndex = 0
        Me.lbl_mail.Text = "Почта"
        '
        'txt_mail
        '
        Me.txt_mail.BackColor = System.Drawing.Color.White
        Me.txt_mail.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.txt_mail.ForeColor = System.Drawing.Color.Black
        Me.txt_mail.Location = New System.Drawing.Point(24, 169)
        Me.txt_mail.MaxLength = 30
        Me.txt_mail.Name = "txt_mail"
        Me.txt_mail.Size = New System.Drawing.Size(237, 30)
        Me.txt_mail.TabIndex = 3
        Me.Hint_User.SetToolTip(Me.txt_mail, "Укажите новую почту")
        '
        'lbl_Nickname
        '
        Me.lbl_Nickname.AutoSize = True
        Me.lbl_Nickname.BackColor = System.Drawing.Color.Black
        Me.lbl_Nickname.Font = New System.Drawing.Font("Comic Sans MS", 14.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_Nickname.ForeColor = System.Drawing.Color.White
        Me.lbl_Nickname.Location = New System.Drawing.Point(19, 78)
        Me.lbl_Nickname.Name = "lbl_Nickname"
        Me.lbl_Nickname.Size = New System.Drawing.Size(95, 26)
        Me.lbl_Nickname.TabIndex = 0
        Me.lbl_Nickname.Text = "Никнейм"
        '
        'txt_Name
        '
        Me.txt_Name.BackColor = System.Drawing.Color.White
        Me.txt_Name.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.txt_Name.ForeColor = System.Drawing.Color.Black
        Me.txt_Name.Location = New System.Drawing.Point(24, 45)
        Me.txt_Name.MaxLength = 20
        Me.txt_Name.Name = "txt_Name"
        Me.txt_Name.Size = New System.Drawing.Size(237, 30)
        Me.txt_Name.TabIndex = 1
        Me.Hint_User.SetToolTip(Me.txt_Name, "Введите своё имя")
        '
        'lbl_Name
        '
        Me.lbl_Name.AutoSize = True
        Me.lbl_Name.BackColor = System.Drawing.Color.Black
        Me.lbl_Name.Font = New System.Drawing.Font("Comic Sans MS", 14.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_Name.ForeColor = System.Drawing.Color.White
        Me.lbl_Name.Location = New System.Drawing.Point(19, 16)
        Me.lbl_Name.Name = "lbl_Name"
        Me.lbl_Name.Size = New System.Drawing.Size(154, 26)
        Me.lbl_Name.TabIndex = 0
        Me.lbl_Name.Text = "Настоящее имя"
        '
        'frm_EditProfile
        '
        Me.AcceptButton = Me.btn_save
        Me.AutoScaleDimensions = New System.Drawing.SizeF(6.0!, 13.0!)
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.BackColor = System.Drawing.Color.Black
        Me.ClientSize = New System.Drawing.Size(617, 384)
        Me.Controls.Add(Me.GB_redakt)
        Me.Controls.Add(Me.pnl_red)
        Me.Controls.Add(Me.trei)
        Me.Controls.Add(Me.EndIt)
        Me.Controls.Add(Me.ChatICO)
        Me.Controls.Add(Me.Stringer)
        Me.ForeColor = System.Drawing.Color.White
        Me.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None
        Me.Icon = CType(resources.GetObject("$this.Icon"), System.Drawing.Icon)
        Me.MaximizeBox = False
        Me.MinimizeBox = False
        Me.Name = "frm_EditProfile"
        Me.Text = "Редактирование профиля"
        CType(Me.trei, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).EndInit()
        Me.pnl_red.ResumeLayout(False)
        Me.pnl_red.PerformLayout()
        Me.GB_redakt.ResumeLayout(False)
        Me.GB_redakt.PerformLayout()
        Me.GB_Pass.ResumeLayout(False)
        Me.GB_Pass.PerformLayout()
        Me.ResumeLayout(False)

    End Sub
    Friend WithEvents trei As System.Windows.Forms.PictureBox
    Friend WithEvents EndIt As System.Windows.Forms.PictureBox
    Friend WithEvents ChatICO As System.Windows.Forms.PictureBox
    Friend WithEvents Stringer As System.Windows.Forms.PictureBox
    Friend WithEvents pnl_red As System.Windows.Forms.Panel
    Friend WithEvents lbl_redakt As System.Windows.Forms.Label
    Friend WithEvents GB_redakt As System.Windows.Forms.GroupBox
    Friend WithEvents mask_phone As System.Windows.Forms.MaskedTextBox
    Friend WithEvents lbl_phone As System.Windows.Forms.Label
    Friend WithEvents txt_Nickname As System.Windows.Forms.TextBox
    Friend WithEvents lbl_mail As System.Windows.Forms.Label
    Friend WithEvents txt_mail As System.Windows.Forms.TextBox
    Friend WithEvents lbl_Nickname As System.Windows.Forms.Label
    Friend WithEvents txt_Name As System.Windows.Forms.TextBox
    Friend WithEvents lbl_Name As System.Windows.Forms.Label
    Friend WithEvents txt_repeatpass As System.Windows.Forms.TextBox
    Friend WithEvents lbl_repeatpass As System.Windows.Forms.Label
    Friend WithEvents txt_pass As System.Windows.Forms.TextBox
    Friend WithEvents lbl_Pass As System.Windows.Forms.Label
    Friend WithEvents GB_Pass As System.Windows.Forms.GroupBox
    Friend WithEvents btn_save As System.Windows.Forms.Button
    Friend WithEvents ChangeBoxX As System.Windows.Forms.CheckBox
    Friend WithEvents Hint_User As ToolTip
End Class
