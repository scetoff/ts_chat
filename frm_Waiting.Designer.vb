<Global.Microsoft.VisualBasic.CompilerServices.DesignerGenerated()> _
Partial Class frm_Waiting
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
        Dim resources As System.ComponentModel.ComponentResourceManager = New System.ComponentModel.ComponentResourceManager(GetType(frm_Waiting))
        Me.lbl_text = New System.Windows.Forms.Label()
        Me.timeVsp = New System.Windows.Forms.Timer(Me.components)
        Me.img_round = New System.Windows.Forms.PictureBox()
        Me.trei = New System.Windows.Forms.PictureBox()
        Me.EndIt = New System.Windows.Forms.PictureBox()
        Me.ChatICO = New System.Windows.Forms.PictureBox()
        Me.Stringer = New System.Windows.Forms.PictureBox()
        Me.Waiten = New System.Windows.Forms.PictureBox()
        Me.pctr_anim = New System.Windows.Forms.PictureBox()
        CType(Me.img_round, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.trei, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.Waiten, System.ComponentModel.ISupportInitialize).BeginInit()
        CType(Me.pctr_anim, System.ComponentModel.ISupportInitialize).BeginInit()
        Me.SuspendLayout()
        '
        'lbl_text
        '
        Me.lbl_text.BackColor = System.Drawing.Color.Transparent
        Me.lbl_text.FlatStyle = System.Windows.Forms.FlatStyle.Flat
        Me.lbl_text.Font = New System.Drawing.Font("Comic Sans MS", 21.75!, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, CType(204, Byte))
        Me.lbl_text.ForeColor = System.Drawing.Color.Gray
        Me.lbl_text.Location = New System.Drawing.Point(70, 218)
        Me.lbl_text.Name = "lbl_text"
        Me.lbl_text.Size = New System.Drawing.Size(223, 79)
        Me.lbl_text.TabIndex = 9
        Me.lbl_text.Text = "Идёт загрузка " & Global.Microsoft.VisualBasic.ChrW(13) & Global.Microsoft.VisualBasic.ChrW(10) & "программы..."
        Me.lbl_text.TextAlign = System.Drawing.ContentAlignment.MiddleCenter
        '
        'timeVsp
        '
        Me.timeVsp.Enabled = True
        Me.timeVsp.Interval = 50
        '
        'img_round
        '
        Me.img_round.Image = Global.TSchat.My.Resources.Resources.step_2
        Me.img_round.Location = New System.Drawing.Point(-51, -47)
        Me.img_round.Name = "img_round"
        Me.img_round.Size = New System.Drawing.Size(450, 450)
        Me.img_round.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.img_round.TabIndex = 10
        Me.img_round.TabStop = False
        '
        'trei
        '
        Me.trei.BackColor = System.Drawing.Color.Navy
        Me.trei.Image = Global.TSchat.My.Resources.Resources.remove
        Me.trei.Location = New System.Drawing.Point(109, 10)
        Me.trei.Name = "trei"
        Me.trei.Size = New System.Drawing.Size(35, 35)
        Me.trei.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.trei.TabIndex = 8
        Me.trei.TabStop = False
        Me.trei.Visible = False
        '
        'EndIt
        '
        Me.EndIt.BackColor = System.Drawing.Color.Navy
        Me.EndIt.Image = Global.TSchat.My.Resources.Resources.clear_search
        Me.EndIt.Location = New System.Drawing.Point(150, 10)
        Me.EndIt.Name = "EndIt"
        Me.EndIt.Size = New System.Drawing.Size(35, 35)
        Me.EndIt.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.EndIt.TabIndex = 7
        Me.EndIt.TabStop = False
        Me.EndIt.Visible = False
        '
        'ChatICO
        '
        Me.ChatICO.BackColor = System.Drawing.Color.Navy
        Me.ChatICO.Image = Global.TSchat.My.Resources.Resources.ICOCOS1
        Me.ChatICO.Location = New System.Drawing.Point(68, 12)
        Me.ChatICO.Name = "ChatICO"
        Me.ChatICO.Size = New System.Drawing.Size(35, 35)
        Me.ChatICO.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.ChatICO.TabIndex = 6
        Me.ChatICO.TabStop = False
        Me.ChatICO.Visible = False
        '
        'Stringer
        '
        Me.Stringer.BackColor = System.Drawing.Color.Navy
        Me.Stringer.Location = New System.Drawing.Point(-1, 1)
        Me.Stringer.Name = "Stringer"
        Me.Stringer.Size = New System.Drawing.Size(250, 46)
        Me.Stringer.TabIndex = 5
        Me.Stringer.TabStop = False
        Me.Stringer.Visible = False
        '
        'Waiten
        '
        Me.Waiten.Image = Global.TSchat.My.Resources.Resources.conwa1t
        Me.Waiten.Location = New System.Drawing.Point(58, 53)
        Me.Waiten.Name = "Waiten"
        Me.Waiten.Size = New System.Drawing.Size(235, 235)
        Me.Waiten.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.Waiten.TabIndex = 0
        Me.Waiten.TabStop = False
        '
        'pctr_anim
        '
        Me.pctr_anim.Image = Global.TSchat.My.Resources.Resources.wait
        Me.pctr_anim.Location = New System.Drawing.Point(108, 25)
        Me.pctr_anim.Name = "pctr_anim"
        Me.pctr_anim.Size = New System.Drawing.Size(140, 83)
        Me.pctr_anim.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage
        Me.pctr_anim.TabIndex = 11
        Me.pctr_anim.TabStop = False
        '
        'frm_Waiting
        '
        Me.AutoScaleDimensions = New System.Drawing.SizeF(6.0!, 13.0!)
        Me.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font
        Me.BackColor = System.Drawing.Color.Black
        Me.ClientSize = New System.Drawing.Size(350, 350)
        Me.Controls.Add(Me.pctr_anim)
        Me.Controls.Add(Me.lbl_text)
        Me.Controls.Add(Me.Waiten)
        Me.Controls.Add(Me.img_round)
        Me.Controls.Add(Me.Stringer)
        Me.Controls.Add(Me.ChatICO)
        Me.Controls.Add(Me.trei)
        Me.Controls.Add(Me.EndIt)
        Me.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None
        Me.Icon = CType(resources.GetObject("$this.Icon"), System.Drawing.Icon)
        Me.Name = "frm_Waiting"
        Me.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen
        Me.Text = "frm_Waiting"
        CType(Me.img_round, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.trei, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.EndIt, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.ChatICO, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.Stringer, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.Waiten, System.ComponentModel.ISupportInitialize).EndInit()
        CType(Me.pctr_anim, System.ComponentModel.ISupportInitialize).EndInit()
        Me.ResumeLayout(False)

    End Sub
    Friend WithEvents trei As PictureBox
    Friend WithEvents EndIt As PictureBox
    Friend WithEvents ChatICO As PictureBox
    Friend WithEvents Stringer As PictureBox
    Friend WithEvents lbl_text As Label
    Friend WithEvents timeVsp As Timer
    Friend WithEvents Waiten As PictureBox
    Friend WithEvents img_round As PictureBox
    Friend WithEvents pctr_anim As PictureBox
End Class
