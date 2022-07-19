<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()> _
Partial Class frm_FriendsProfile
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
        Dim resources As System.ComponentModel.ComponentResourceManager = New System.ComponentModel.ComponentResourceManager(GetType(frm_FriendsProfile))
        Me.pnl_frnd = New System.Windows.Forms.Panel()
        Me.lbl_frnd = New System.Windows.Forms.Label()
        Me.GB_Frnd = New System.Windows.Forms.GroupBox()
        Me.lbl_Send = New System.Windows.Forms.Label()
        Me.GB_Info = New System.Windows.Forms.GroupBox()
        Me.lbl_PhoneS = New System.Windows.Forms.Label()
        Me.lbl_MailS = New System.Windows.Forms.Label()
        Me.lbl_NicknameS = New System.Windows.Forms.Label()
        Me.lbl_NameS = New System.Windows.Forms.Label()
        Me.lbl_phone = New System.Windows.Forms.Label()
        Me.lbl_Name = New System.Windows.Forms.Label()
        Me.lbl_mail = New System.Windows.Forms.Label()
        Me.lbl_Nickname = New System.Windows.Forms.Label()
        Me.sendmes_pic = New System.Windows.Forms.PictureBox()
        Me.Friend_Picture = New System.Windows.Forms.PictureBox()
        Me.trei = New System.Windows.Forms.PictureBox()
        Me.EndIt = New System.Windows.Forms.PictureBox()
        Me.ChatICO = New System.Windows.Forms.PictureBox()
        Me.Stringer = New System.Windows.Forms.PictureBox()
        Me.pnl_frnd.SuspendLayout()
        Me.GB_Frnd.SuspendLayout()
        Me.GB_Info.SuspendLayout()
        CType(Me.sendmes_pic, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.Friend_Picture, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.trei, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).BeginInit()
        Me.SuspendLayout()
        '
        'pnl_frnd
        '
        Me.pnl_frnd.BackColor = System.Drawing.Color.Black
        Me.pnl_frnd.Controls.Add(Me.lbl_frnd)
        Me.pnl_frnd.Location = New System.Drawing.Point(-3, 36)
        Me.pnl_frnd.Name = "pnl_frnd"
        Me.pnl_frnd.Size = New System.Drawing.Size(602, 36)
        Me.pnl_frnd.TabIndex = 19
        '
        'lbl_frnd
        '
        Me.lbl_frnd.AutoSize = True
        Me.lbl_frnd.BackColor = System.Drawing.Color.Black
        Me.lbl_frnd.Font = New System.Drawing.Font("Comic Sans MS", 14.25!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_frnd.ForeColor = System.Drawing.Color.White
        Me.lbl_frnd.Location = New System.Drawing.Point(12, 4)
        Me.lbl_frnd.Name = "lbl_frnd"
        Me.lbl_frnd.Size = New System.Drawing.Size(154, 26)
        Me.lbl_frnd.TabIndex = 0
        Me.lbl_frnd.Text = "Профиль друга"
        '
        'GB_Frnd
        '
        Me.GB_Frnd.Controls.Add(Me.lbl_Send)
        Me.GB_Frnd.Controls.Add(Me.GB_Info)
        Me.GB_Frnd.Controls.Add(Me.sendmes_pic)
        Me.GB_Frnd.Controls.Add(Me.Friend_Picture)
        Me.GB_Frnd.Location = New System.Drawing.Point(13, 78)
        Me.GB_Frnd.Name = "GB_Frnd"
        Me.GB_Frnd.Size = New System.Drawing.Size(569, 266)
        Me.GB_Frnd.TabIndex = 0
        Me.GB_Frnd.TabStop = False
        '
        'lbl_Send
        '
        Me.lbl_Send.AutoSize = True
        Me.lbl_Send.BackColor = System.Drawing.Color.Black
        Me.lbl_Send.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_Send.ForeColor = System.Drawing.Color.White
        Me.lbl_Send.Location = New System.Drawing.Point(8, 228)
        Me.lbl_Send.Name = "lbl_Send"
        Me.lbl_Send.Size = New System.Drawing.Size(179, 23)
        Me.lbl_Send.TabIndex = 0
        Me.lbl_Send.Text = "Отправить сообщение"
        '
        'GB_Info
        '
        Me.GB_Info.Controls.Add(Me.lbl_PhoneS)
        Me.GB_Info.Controls.Add(Me.lbl_MailS)
        Me.GB_Info.Controls.Add(Me.lbl_NicknameS)
        Me.GB_Info.Controls.Add(Me.lbl_NameS)
        Me.GB_Info.Controls.Add(Me.lbl_phone)
        Me.GB_Info.Controls.Add(Me.lbl_Name)
        Me.GB_Info.Controls.Add(Me.lbl_mail)
        Me.GB_Info.Controls.Add(Me.lbl_Nickname)
        Me.GB_Info.Location = New System.Drawing.Point(190, 13)
        Me.GB_Info.Name = "GB_Info"
        Me.GB_Info.Size = New System.Drawing.Size(363, 238)
        Me.GB_Info.TabIndex = 0
        Me.GB_Info.TabStop = False
        '
        'lbl_PhoneS
        '
        Me.lbl_PhoneS.AutoSize = True
        Me.lbl_PhoneS.BackColor = System.Drawing.Color.Black
        Me.lbl_PhoneS.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_PhoneS.ForeColor = System.Drawing.Color.White
        Me.lbl_PhoneS.Location = New System.Drawing.Point(144, 202)
        Me.lbl_PhoneS.Name = "lbl_PhoneS"
        Me.lbl_PhoneS.Size = New System.Drawing.Size(78, 23)
        Me.lbl_PhoneS.TabIndex = 27
        Me.lbl_PhoneS.Text = "Телефон"
        '
        'lbl_MailS
        '
        Me.lbl_MailS.AutoSize = True
        Me.lbl_MailS.BackColor = System.Drawing.Color.Black
        Me.lbl_MailS.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_MailS.ForeColor = System.Drawing.Color.White
        Me.lbl_MailS.Location = New System.Drawing.Point(144, 108)
        Me.lbl_MailS.Name = "lbl_MailS"
        Me.lbl_MailS.Size = New System.Drawing.Size(59, 23)
        Me.lbl_MailS.TabIndex = 28
        Me.lbl_MailS.Text = "Почта"
        '
        'lbl_NicknameS
        '
        Me.lbl_NicknameS.AutoSize = True
        Me.lbl_NicknameS.BackColor = System.Drawing.Color.Black
        Me.lbl_NicknameS.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_NicknameS.ForeColor = System.Drawing.Color.White
        Me.lbl_NicknameS.Location = New System.Drawing.Point(144, 64)
        Me.lbl_NicknameS.Name = "lbl_NicknameS"
        Me.lbl_NicknameS.Size = New System.Drawing.Size(79, 23)
        Me.lbl_NicknameS.TabIndex = 25
        Me.lbl_NicknameS.Text = "Никнейм"
        '
        'lbl_NameS
        '
        Me.lbl_NameS.AutoSize = True
        Me.lbl_NameS.BackColor = System.Drawing.Color.Black
        Me.lbl_NameS.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_NameS.ForeColor = System.Drawing.Color.White
        Me.lbl_NameS.Location = New System.Drawing.Point(144, 16)
        Me.lbl_NameS.Name = "lbl_NameS"
        Me.lbl_NameS.Size = New System.Drawing.Size(127, 23)
        Me.lbl_NameS.TabIndex = 26
        Me.lbl_NameS.Text = "Настоящее имя"
        '
        'lbl_phone
        '
        Me.lbl_phone.AutoSize = True
        Me.lbl_phone.BackColor = System.Drawing.Color.Black
        Me.lbl_phone.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_phone.ForeColor = System.Drawing.Color.White
        Me.lbl_phone.Location = New System.Drawing.Point(6, 202)
        Me.lbl_phone.Name = "lbl_phone"
        Me.lbl_phone.Size = New System.Drawing.Size(78, 23)
        Me.lbl_phone.TabIndex = 23
        Me.lbl_phone.Text = "Телефон"
        '
        'lbl_Name
        '
        Me.lbl_Name.AutoSize = True
        Me.lbl_Name.BackColor = System.Drawing.Color.Black
        Me.lbl_Name.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_Name.ForeColor = System.Drawing.Color.White
        Me.lbl_Name.Location = New System.Drawing.Point(6, 16)
        Me.lbl_Name.Name = "lbl_Name"
        Me.lbl_Name.Size = New System.Drawing.Size(127, 23)
        Me.lbl_Name.TabIndex = 22
        Me.lbl_Name.Text = "Настоящее имя"
        '
        'lbl_mail
        '
        Me.lbl_mail.AutoSize = True
        Me.lbl_mail.BackColor = System.Drawing.Color.Black
        Me.lbl_mail.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_mail.ForeColor = System.Drawing.Color.White
        Me.lbl_mail.Location = New System.Drawing.Point(6, 108)
        Me.lbl_mail.Name = "lbl_mail"
        Me.lbl_mail.Size = New System.Drawing.Size(59, 23)
        Me.lbl_mail.TabIndex = 24
        Me.lbl_mail.Text = "Почта"
        '
        'lbl_Nickname
        '
        Me.lbl_Nickname.AutoSize = True
        Me.lbl_Nickname.BackColor = System.Drawing.Color.Black
        Me.lbl_Nickname.Font = New System.Drawing.Font("Comic Sans MS", 12.0!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_Nickname.ForeColor = System.Drawing.Color.White
        Me.lbl_Nickname.Location = New System.Drawing.Point(6, 64)
        Me.lbl_Nickname.Name = "lbl_Nickname"
        Me.lbl_Nickname.Size = New System.Drawing.Size(79, 23)
        Me.lbl_Nickname.TabIndex = 21
        Me.lbl_Nickname.Text = "Никнейм"
        '
        'sendmes_pic
        '
        Me.sendmes_pic.BackColor = System.Drawing.Color.Transparent
        Me.sendmes_pic.Image = Global.TSchat.My.Resources.Resources.Send_mail
        Me.sendmes_pic.Location = New System.Drawing.Point(69, 175)
        Me.sendmes_pic.Name = "sendmes_pic"
        Me.sendmes_pic.Size = New System.Drawing.Size(50, 50)
        Me.sendmes_pic.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.sendmes_pic.TabIndex = 20
        Me.sendmes_pic.TabStop = False
        '
        'Friend_Picture
        '
        Me.Friend_Picture.Location = New System.Drawing.Point(21, 19)
        Me.Friend_Picture.Name = "Friend_Picture"
        Me.Friend_Picture.Size = New System.Drawing.Size(150, 150)
        Me.Friend_Picture.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.Friend_Picture.TabIndex = 0
        Me.Friend_Picture.TabStop = False
        '
        'trei
        '
        Me.trei.BackColor = System.Drawing.Color.Navy
        Me.trei.Image = Global.TSchat.My.Resources.Resources.remove
        Me.trei.Location = New System.Drawing.Point(516, 0)
        Me.trei.Name = "trei"
        Me.trei.Size = New System.Drawing.Size(35, 35)
        Me.trei.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.trei.TabIndex = 18
        Me.trei.TabStop = False
        '
        'EndIt
        '
        Me.EndIt.BackColor = System.Drawing.Color.Navy
        Me.EndIt.Image = Global.TSchat.My.Resources.Resources.clear_search
        Me.EndIt.Location = New System.Drawing.Point(557, 0)
        Me.EndIt.Name = "EndIt"
        Me.EndIt.Size = New System.Drawing.Size(35, 35)
        Me.EndIt.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.EndIt.TabIndex = 17
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
        Me.ChatICO.TabIndex = 16
        Me.ChatICO.TabStop = False
        '
        'Stringer
        '
        Me.Stringer.BackColor = System.Drawing.Color.Navy
        Me.Stringer.Location = New System.Drawing.Point(-3, 0)
        Me.Stringer.Name = "Stringer"
        Me.Stringer.Size = New System.Drawing.Size(602, 35)
        Me.Stringer.TabIndex = 15
        Me.Stringer.TabStop = False
        '
        'frm_FriendsProfile
        '
        Me.AutoScaleDimensions = New System.Drawing.SizeF(6.0!, 13.0!)
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.BackColor = System.Drawing.Color.Black
        Me.ClientSize = New System.Drawing.Size(595, 356)
        Me.Controls.Add(Me.GB_Frnd)
        Me.Controls.Add(Me.pnl_frnd)
        Me.Controls.Add(Me.trei)
        Me.Controls.Add(Me.EndIt)
        Me.Controls.Add(Me.ChatICO)
        Me.Controls.Add(Me.Stringer)
        Me.ForeColor = System.Drawing.Color.White
        Me.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None
        Me.Icon = CType(resources.GetObject("$this.Icon"), System.Drawing.Icon)
        Me.MaximizeBox = False
        Me.MinimizeBox = False
        Me.Name = "frm_FriendsProfile"
        Me.Text = "Профиль друга"
        Me.pnl_frnd.ResumeLayout(False)
        Me.pnl_frnd.PerformLayout()
        Me.GB_Frnd.ResumeLayout(False)
        Me.GB_Frnd.PerformLayout()
        Me.GB_Info.ResumeLayout(False)
        Me.GB_Info.PerformLayout()
        CType(Me.sendmes_pic, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.Friend_Picture, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.trei, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).EndInit()
        Me.ResumeLayout(False)

    End Sub
    Friend WithEvents trei As System.Windows.Forms.PictureBox
    Friend WithEvents EndIt As System.Windows.Forms.PictureBox
    Friend WithEvents ChatICO As System.Windows.Forms.PictureBox
    Friend WithEvents pnl_frnd As System.Windows.Forms.Panel
    Friend WithEvents lbl_frnd As System.Windows.Forms.Label
    Friend WithEvents GB_Frnd As System.Windows.Forms.GroupBox
    Friend WithEvents sendmes_pic As System.Windows.Forms.PictureBox
    Friend WithEvents Friend_Picture As System.Windows.Forms.PictureBox
    Friend WithEvents lbl_Send As System.Windows.Forms.Label
    Friend WithEvents GB_Info As System.Windows.Forms.GroupBox
    Friend WithEvents lbl_PhoneS As System.Windows.Forms.Label
    Friend WithEvents lbl_MailS As System.Windows.Forms.Label
    Friend WithEvents lbl_NicknameS As System.Windows.Forms.Label
    Friend WithEvents lbl_NameS As System.Windows.Forms.Label
    Friend WithEvents lbl_phone As System.Windows.Forms.Label
    Friend WithEvents lbl_Name As System.Windows.Forms.Label
    Friend WithEvents lbl_mail As System.Windows.Forms.Label
    Friend WithEvents lbl_Nickname As System.Windows.Forms.Label
    Friend WithEvents Stringer As PictureBox
End Class
