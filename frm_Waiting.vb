Imports System.Drawing.Drawing2D
Public Class frm_Waiting
    Public Sub New()
        InitializeComponent()
        Me.FormBorderStyle = FormBorderStyle.None
    End Sub

    Dim Index As Integer = 0
    Private Const AW_BLEND = &H80000  'Uses a fade effect. This flag can be used only if hwnd is a top-level window.
    Private Declare Function AnimateWindow Lib "user32" (ByVal hwnd As Int32, ByVal dwTime As Int32, ByVal dwFlags As Int32) As Boolean

    Dim winHide As Integer = &H10000
    Dim winBlend As Integer = &H80000
    Dim AW_ACTIVATE As Integer = &H20000

    'Переменные для перемещния формы
    Dim newPoint As New System.Drawing.Point()
    Dim a As Integer
    Dim b As Integer

    Private Sub frm_Waiting_MouseDown(sender As Object, e As MouseEventArgs) Handles Me.MouseDown
        a = Control.MousePosition.X - Me.Location.X
        b = Control.MousePosition.Y - Me.Location.Y
    End Sub

    Private Sub frm_Waiting_MouseMove(sender As Object, e As MouseEventArgs) Handles Me.MouseMove
        If e.Button = MouseButtons.Left Then
            newPoint = Control.MousePosition
            newPoint.X = newPoint.X - (a)
            newPoint.Y = newPoint.Y - (b)
            Me.Location = newPoint
        End If
    End Sub
    Private Sub Stringer_MouseDown(ByVal sender As Object, ByVal e As System.Windows.Forms.MouseEventArgs) Handles Stringer.MouseDown
        a = Control.MousePosition.X - Me.Location.X
        b = Control.MousePosition.Y - Me.Location.Y
    End Sub

    Private Sub Stringer_MouseMove(ByVal sender As Object, ByVal e As System.Windows.Forms.MouseEventArgs) Handles Stringer.MouseMove
        If e.Button = MouseButtons.Left Then
            newPoint = Control.MousePosition
            newPoint.X = newPoint.X - (a)
            newPoint.Y = newPoint.Y - (b)
            Me.Location = newPoint
        End If
    End Sub

    Private Sub EndIt_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles EndIt.Click
        AnimateWindow(Me.Handle.ToInt32, CInt(500), winHide Or winBlend)
        Me.Close()
    End Sub

    Private Sub trei_Click(ByVal sender As System.Object, ByVal e As System.EventArgs) Handles trei.Click
        Me.WindowState = FormWindowState.Minimized
    End Sub

    Private Sub timeVsp_Tick(sender As Object, e As EventArgs) Handles timeVsp.Tick
        Index = Index + 1
        If Index = 20 Then
            frm_Auth.Show()
            Me.Close()
        End If
    End Sub

    Private Sub frm_Waiting_Load(sender As Object, e As EventArgs) Handles Me.Load

        'AnimateWindow(Me.Handle.ToInt32, CInt(500), AW_ACTIVATE Or winBlend)

        Dim g As New GraphicsPath
        Dim gr As Graphics = Me.CreateGraphics()
        Dim Frm As Rectangle = Me.ClientRectangle
        Frm.Inflate(-1, -1)
        g.AddEllipse(Frm)
        Me.Region = New Region(g)
        gr.Dispose()

        timeVsp.Interval = 500
        timeVsp.Start()
    End Sub
End Class