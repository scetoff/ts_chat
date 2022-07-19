<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()>
Partial Class frm_Auth
    Inherits System.Windows.Forms.Form

    'Form overrides dispose to clean up the component list.
    <System.Diagnostics.DebuggerNonUserCode()>
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
    <System.Diagnostics.DebuggerStepThrough()>
    Private Sub InitializeComponent()
        Me.components = New System.ComponentModel.Container()
        Dim resources As System.ComponentModel.ComponentResourceManager = New System.ComponentModel.ComponentResourceManager(GetType(frm_Auth))
        Me.lblstatus = New System.Windows.Forms.Label()
        Me.btnSignin = New System.Windows.Forms.Button()
        Me.pass = New System.Windows.Forms.TextBox()
        Me.user = New System.Windows.Forms.TextBox()
        Me.email = New System.Windows.Forms.TextBox()
        Me.GB_Login = New System.Windows.Forms.GroupBox()
        Me.switch_day = New System.Windows.Forms.PictureBox()
        Me.PB_ICQ = New System.Windows.Forms.PictureBox()
        Me.lbl_acnt = New System.Windows.Forms.Label()
        Me.lbl_correct2 = New System.Windows.Forms.Label()
        Me.lbl_correct = New System.Windows.Forms.Label()
        Me.lbl_Pass = New System.Windows.Forms.Label()
        Me.lbl_Login = New System.Windows.Forms.Label()
        Me.switch_night = New System.Windows.Forms.PictureBox()
        Me.GB_Register = New System.Windows.Forms.GroupBox()
        Me.lbl_Mail = New System.Windows.Forms.Label()
        Me.lbl_PassCurrent = New System.Windows.Forms.Label()
        Me.lbl_Name = New System.Windows.Forms.Label()
        Me.Hint_User = New System.Windows.Forms.ToolTip(Me.components)
        Me.Checker = New System.Windows.Forms.CheckBox()
        Me.trei = New System.Windows.Forms.PictureBox()
        Me.EndIt = New System.Windows.Forms.PictureBox()
        Me.ChatICO = New System.Windows.Forms.PictureBox()
        Me.Stringer = New System.Windows.Forms.PictureBox()
        Me.logo = New System.Windows.Forms.PictureBox()
        Me.tav_img = New System.Windows.Forms.PictureBox()
        Me.GB_Login.SuspendLayout()
        CType(Me.switch_day, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.PB_ICQ, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.switch_night, System.ComponentModel.ISupportInitialize).BeginInit()
        Me.GB_Register.SuspendLayout()
        CType(Me.trei, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.logo, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.tav_img, System.ComponentModel.ISupportInitialize).BeginInit()
        Me.SuspendLayout()
        '
        'lblstatus
        '
        resources.ApplyResources(Me.lblstatus, "lblstatus")
        Me.lblstatus.ForeColor = System.Drawing.Color.SeaGreen
        Me.lblstatus.Name = "lblstatus"
        '
        'btnSignin
        '
        Me.btnSignin.BackColor = System.Drawing.Color.Black
        Me.btnSignin.Cursor = System.Windows.Forms.Cursors.Hand
        Me.btnSignin.FlatAppearance.BorderColor = System.Drawing.Color.White
        Me.btnSignin.FlatAppearance.MouseDownBackColor = System.Drawing.Color.Transparent
        Me.btnSignin.FlatAppearance.MouseOverBackColor = System.Drawing.Color.Transparent
        resources.ApplyResources(Me.btnSignin, "btnSignin")
        Me.btnSignin.ForeColor = System.Drawing.Color.White
        Me.btnSignin.Name = "btnSignin"
        Me.btnSignin.UseVisualStyleBackColor = False
        '
        'pass
        '
        Me.pass.BackColor = System.Drawing.Color.White
        Me.pass.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle
        resources.ApplyResources(Me.pass, "pass")
        Me.pass.ForeColor = System.Drawing.SystemColors.ActiveCaptionText
        Me.pass.Name = "pass"
        Me.Hint_User.SetToolTip(Me.pass, resources.GetString("pass.ToolTip"))
        '
        'user
        '
        Me.user.BackColor = System.Drawing.Color.White
        Me.user.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle
        resources.ApplyResources(Me.user, "user")
        Me.user.ForeColor = System.Drawing.SystemColors.ActiveCaptionText
        Me.user.Name = "user"
        Me.Hint_User.SetToolTip(Me.user, resources.GetString("user.ToolTip"))
        '
        'email
        '
        Me.email.BackColor = System.Drawing.Color.White
        Me.email.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle
        resources.ApplyResources(Me.email, "email")
        Me.email.ForeColor = System.Drawing.SystemColors.ActiveCaptionText
        Me.email.Name = "email"
        Me.Hint_User.SetToolTip(Me.email, resources.GetString("email.ToolTip"))
        '
        'GB_Login
        '
        Me.GB_Login.BackColor = System.Drawing.Color.Black
        Me.GB_Login.Controls.Add(Me.PB_ICQ)
        Me.GB_Login.Controls.Add(Me.lbl_acnt)
        Me.GB_Login.Controls.Add(Me.lbl_correct2)
        Me.GB_Login.Controls.Add(Me.lbl_correct)
        Me.GB_Login.Controls.Add(Me.lbl_Pass)
        Me.GB_Login.Controls.Add(Me.lbl_Login)
        Me.GB_Login.Controls.Add(Me.btnSignin)
        Me.GB_Login.Controls.Add(Me.user)
        Me.GB_Login.Controls.Add(Me.lblstatus)
        Me.GB_Login.Controls.Add(Me.pass)
        Me.GB_Login.Controls.Add(Me.switch_day)
        Me.GB_Login.Controls.Add(Me.switch_night)
        Me.GB_Login.FlatStyle = System.Windows.Forms.FlatStyle.Flat
        resources.ApplyResources(Me.GB_Login, "GB_Login")
        Me.GB_Login.ForeColor = System.Drawing.Color.White
        Me.GB_Login.Name = "GB_Login"
        Me.GB_Login.TabStop = False
        '
        'switch_day
        '
        Me.switch_day.Image = Global.TSchat.My.Resources.Resources.b_day
        resources.ApplyResources(Me.switch_day, "switch_day")
        Me.switch_day.Name = "switch_day"
        Me.switch_day.TabStop = False
        '
        'PB_ICQ
        '
        Me.PB_ICQ.Image = Global.TSchat.My.Resources.Resources.icq_logo
        resources.ApplyResources(Me.PB_ICQ, "PB_ICQ")
        Me.PB_ICQ.Name = "PB_ICQ"
        Me.PB_ICQ.TabStop = False
        Me.Hint_User.SetToolTip(Me.PB_ICQ, resources.GetString("PB_ICQ.ToolTip"))
        '
        'lbl_acnt
        '
        resources.ApplyResources(Me.lbl_acnt, "lbl_acnt")
        Me.lbl_acnt.BorderStyle = System.Windows.Forms.BorderStyle.FixedSingle
        Me.lbl_acnt.Cursor = System.Windows.Forms.Cursors.Hand
        Me.lbl_acnt.ForeColor = System.Drawing.Color.White
        Me.lbl_acnt.Name = "lbl_acnt"
        '
        'lbl_correct2
        '
        resources.ApplyResources(Me.lbl_correct2, "lbl_correct2")
        Me.lbl_correct2.Name = "lbl_correct2"
        '
        'lbl_correct
        '
        resources.ApplyResources(Me.lbl_correct, "lbl_correct")
        Me.lbl_correct.Name = "lbl_correct"
        '
        'lbl_Pass
        '
        resources.ApplyResources(Me.lbl_Pass, "lbl_Pass")
        Me.lbl_Pass.ForeColor = System.Drawing.Color.White
        Me.lbl_Pass.Name = "lbl_Pass"
        '
        'lbl_Login
        '
        resources.ApplyResources(Me.lbl_Login, "lbl_Login")
        Me.lbl_Login.ForeColor = System.Drawing.Color.White
        Me.lbl_Login.Name = "lbl_Login"
        '
        'switch_night
        '
        Me.switch_night.Image = Global.TSchat.My.Resources.Resources.b_night
        resources.ApplyResources(Me.switch_night, "switch_night")
        Me.switch_night.Name = "switch_night"
        Me.switch_night.TabStop = False
        '
        'GB_Register
        '
        Me.GB_Register.BackColor = System.Drawing.Color.Black
        Me.GB_Register.Controls.Add(Me.lbl_Mail)
        Me.GB_Register.Controls.Add(Me.lbl_PassCurrent)
        Me.GB_Register.Controls.Add(Me.lbl_Name)
        Me.GB_Register.Controls.Add(Me.email)
        resources.ApplyResources(Me.GB_Register, "GB_Register")
        Me.GB_Register.ForeColor = System.Drawing.Color.White
        Me.GB_Register.Name = "GB_Register"
        Me.GB_Register.TabStop = False
        '
        'lbl_Mail
        '
        resources.ApplyResources(Me.lbl_Mail, "lbl_Mail")
        Me.lbl_Mail.ForeColor = System.Drawing.Color.White
        Me.lbl_Mail.Name = "lbl_Mail"
        '
        'lbl_PassCurrent
        '
        resources.ApplyResources(Me.lbl_PassCurrent, "lbl_PassCurrent")
        Me.lbl_PassCurrent.ForeColor = System.Drawing.Color.White
        Me.lbl_PassCurrent.Name = "lbl_PassCurrent"
        '
        'lbl_Name
        '
        resources.ApplyResources(Me.lbl_Name, "lbl_Name")
        Me.lbl_Name.BackColor = System.Drawing.Color.Black
        Me.lbl_Name.ForeColor = System.Drawing.Color.White
        Me.lbl_Name.Name = "lbl_Name"
        '
        'Checker
        '
        resources.ApplyResources(Me.Checker, "Checker")
        Me.Checker.Name = "Checker"
        Me.Checker.UseVisualStyleBackColor = True
        '
        'trei
        '
        Me.trei.BackColor = System.Drawing.Color.Navy
        Me.trei.Image = Global.TSchat.My.Resources.Resources.remove
        resources.ApplyResources(Me.trei, "trei")
        Me.trei.Name = "trei"
        Me.trei.TabStop = False
        '
        'EndIt
        '
        Me.EndIt.BackColor = System.Drawing.Color.Navy
        Me.EndIt.Image = Global.TSchat.My.Resources.Resources.clear_search
        resources.ApplyResources(Me.EndIt, "EndIt")
        Me.EndIt.Name = "EndIt"
        Me.EndIt.TabStop = False
        '
        'ChatICO
        '
        Me.ChatICO.BackColor = System.Drawing.Color.Navy
        Me.ChatICO.Image = Global.TSchat.My.Resources.Resources.ICOCOS1
        resources.ApplyResources(Me.ChatICO, "ChatICO")
        Me.ChatICO.Name = "ChatICO"
        Me.ChatICO.TabStop = False
        '
        'Stringer
        '
        Me.Stringer.BackColor = System.Drawing.Color.Navy
        resources.ApplyResources(Me.Stringer, "Stringer")
        Me.Stringer.Name = "Stringer"
        Me.Stringer.TabStop = False
        '
        'logo
        '
        Me.logo.Image = Global.TSchat.My.Resources.Resources._32323
        resources.ApplyResources(Me.logo, "logo")
        Me.logo.Name = "logo"
        Me.logo.TabStop = False
        '
        'tav_img
        '
        Me.tav_img.Image = Global.TSchat.My.Resources.Resources._222
        resources.ApplyResources(Me.tav_img, "tav_img")
        Me.tav_img.Name = "tav_img"
        Me.tav_img.TabStop = False
        '
        'frm_Auth
        '
        resources.ApplyResources(Me, "$this")
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.BackColor = System.Drawing.Color.Black
        Me.Controls.Add(Me.Checker)
        Me.Controls.Add(Me.GB_Login)
        Me.Controls.Add(Me.trei)
        Me.Controls.Add(Me.EndIt)
        Me.Controls.Add(Me.ChatICO)
        Me.Controls.Add(Me.Stringer)
        Me.Controls.Add(Me.GB_Register)
        Me.Controls.Add(Me.logo)
        Me.Controls.Add(Me.tav_img)
        Me.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None
        Me.MaximizeBox = False
        Me.MinimizeBox = False
        Me.Name = "frm_Auth"
        Me.GB_Login.ResumeLayout(False)
        Me.GB_Login.PerformLayout()
        CType(Me.switch_day, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.PB_ICQ, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.switch_night, System.ComponentModel.ISupportInitialize).EndInit()
        Me.GB_Register.ResumeLayout(False)
        Me.GB_Register.PerformLayout()
        CType(Me.trei, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.logo, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.tav_img, System.ComponentModel.ISupportInitialize).EndInit()
        Me.ResumeLayout(False)
        Me.PerformLayout()

    End Sub
    Friend WithEvents Stringer As System.Windows.Forms.PictureBox
    Friend WithEvents ChatICO As System.Windows.Forms.PictureBox
    Friend WithEvents EndIt As System.Windows.Forms.PictureBox
    Friend WithEvents trei As System.Windows.Forms.PictureBox
    Friend WithEvents lblstatus As Label
    Friend WithEvents btnSignin As Button
    Friend WithEvents pass As TextBox
    Friend WithEvents user As TextBox
    Friend WithEvents email As TextBox
    Friend WithEvents GB_Login As GroupBox
    Friend WithEvents lbl_correct2 As Label
    Friend WithEvents lbl_correct As Label
    Friend WithEvents lbl_Pass As Label
    Friend WithEvents lbl_Login As Label
    Friend WithEvents logo As PictureBox
    Friend WithEvents GB_Register As GroupBox
    Friend WithEvents lbl_Mail As Label
    Friend WithEvents lbl_PassCurrent As Label
    Friend WithEvents lbl_Name As Label
    Friend WithEvents tav_img As PictureBox
    Friend WithEvents lbl_acnt As Label
    Friend WithEvents PB_ICQ As PictureBox
    Friend WithEvents Hint_User As ToolTip
    Friend WithEvents Checker As CheckBox
    Friend WithEvents switch_day As PictureBox
    Friend WithEvents switch_night As PictureBox
End Class
